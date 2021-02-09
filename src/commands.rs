use std::env::set_current_dir;
use std::fs::{read_to_string, remove_file, write};
use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use std::sync::mpsc::channel;

use chrono::offset::Local;
use colored::Colorize;
use convert_case::{Case, Casing};
use notify::{op, raw_watcher, RawEvent, RecursiveMode, Watcher};

use crate::config::Configuration;
use crate::definitions::CargoToml;
use crate::utils::{
  check_if_lib_dir, get_absolute_path, get_dynamic_libraries_path, get_dynamic_library_ext,
  get_insert_location, get_library_name, get_project_toml_as_object, is_module_in_project_toml,
  print_to_console, set_project_toml_contents, write_and_fmt, ConsoleColors,
};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// Creates the library used to manage Rust modules.
///
/// # Arguments
///
/// `name`              - The name of the library.
/// `godot_project_dir` - The relative path to the directory of the Godot project that this library of modules is for.
pub fn command_new(name: &str, godot_project_dir: PathBuf) {
  print_to_console("Creating library", ConsoleColors::WHITE);

  // Normalize the library name so we can be sure of it's formatting.
  let library_name_normalized = name.to_case(Case::Snake);

  // Get the absolute path to the library and Godot project if it isn't already
  // one.
  let library_absolute_path = get_absolute_path(PathBuf::from(&name));
  let godot_absolute_path = get_absolute_path(godot_project_dir);

  if library_absolute_path.exists() {
    // If there's already a directory with the library name then we print an
    // error to the console and exit early.
    print_to_console(
      "Cannot create library, directory with the same name already exists",
      ConsoleColors::RED,
    );
    exit(1);
  }

  if !godot_absolute_path.join("project.godot").exists() {
    // If there's not a project.godot file at the root of the provided Godot
    // project directory then we print an error to the console and exit early.
    print_to_console(
      "The Godot project dir provided is not valid",
      ConsoleColors::RED,
    );
    exit(1);
  }

  // Run the `cargo new --lib` command to create the library.
  match Command::new("cargo")
    .arg("new")
    .arg(&library_name_normalized)
    .arg("--lib")
    .output()
  {
    Ok(_v) => (),
    Err(e) => {
      print_to_console(&e.to_string(), ConsoleColors::RED);
      exit(1);
    }
  }

  // Change to the library directory so that we can work with the Cargo.toml
  // and set up our dependencies.
  set_current_dir(&library_name_normalized).expect("Unable to change to library directory");

  // Get the base Cargo.toml contents of the library.
  let library_cargo_toml_string =
    read_to_string("Cargo.toml").expect("Unable to read library's Cargo.toml file");

  // Add the necessary dependencies to the base contents.
  let new_library_cargo_toml: CargoToml = toml::from_str(&library_cargo_toml_string)
    .expect("Unable to parse the library's Cargo.toml file");

  // Turn the new contents of the library's Cargo.toml into a string so that we
  // can write it back to the library. We also need to normalize some things here
  // because when we turn the Cargo toml contents to a string, extra symbols get
  // added.
  let new_library_cargo_toml_string = toml::to_string(&new_library_cargo_toml)
    .expect("Unable to convert the library's new Cargo.toml to a string")
    .replace("\\", "")
    .replace("\"{", "{")
    .replace("}\"", "}");

  // Next we overwrite the contents of the Cargo.toml file with our contents.
  write("Cargo.toml", new_library_cargo_toml_string)
    .expect("Unable to update contents of the library's Cargo.toml file");

  // Create the project.toml config file and write it to the library
  // directory, returning early if there was a problem.
  let config = Configuration {
    godot_project_name: godot_absolute_path
      .file_name()
      .unwrap()
      .to_str()
      .expect("Unable to convert Godot file name to str")
      .to_string(),
    modules: vec![],
  };
  let config_string = toml::to_string(&config).expect("Unable to convert config to string");
  write("project.toml", config_string).expect("Unable to create project.toml file");

  // Create the initial src/lib.rs file in the library.
  let lib_template = include_str!("./templates/lib.rs");

  write_and_fmt("src/lib.rs", lib_template).expect("Unable to create the initial lib.rs file");

  // Create the `rust_modules` directory within the Godot project.
  match std::fs::create_dir_all(&godot_absolute_path.join("rust_modules")) {
    Ok(_) => (),
    Err(e) => {
      // If there was a problem creating the directory then we print the error
      // to the console and exit early.
      print_to_console(&e.to_string(), ConsoleColors::RED);
      exit(1);
    }
  }

  // Create the contents for the gndlib file that goes in the Godot project to
  // specifies the dynamic libraries that should be used.
  let gdnlib_template = include_str!("./templates/gdnlib.txt");
  let gdnlib_with_library_name = gdnlib_template.replace("LIBRARY_NAME", &library_name_normalized);
  let gdnlib_filename = format!("{}.gdnlib", &library_name_normalized);
  write(
    &godot_absolute_path.join(gdnlib_filename),
    gdnlib_with_library_name,
  )
  .expect("Unable to create gdnlib file");

  print_to_console("library created", ConsoleColors::GREEN);
}

/// Creates a new Rust module inside the library.
///
/// # Arguments
///
/// `name`      - The name of the module to create. The module name should be PascalCase with examples including 'Player', 'Princess', 'Mob', etc.
/// `is_plugin` - Indicates whether the module is for a plugin or not.
pub fn command_create(name: &str, is_plugin: bool) {
  print_to_console("Creating module", ConsoleColors::WHITE);

  // Make sure that we are in the library directory by checking for the presence
  // of a project.json file.
  check_if_lib_dir();

  // Get the contents of the project.toml file.
  let mut project_toml = get_project_toml_as_object();

  // We need various different casings of the module name.
  let module_name_snake_case = &name.to_case(Case::Snake);
  let module_name_pascal_case = &name.to_case(Case::Pascal);

  // Check if the module already exists.
  let module_exists = is_module_in_project_toml(&project_toml.modules, &module_name_pascal_case);
  if module_exists {
    print_to_console(
      "A module with the same name already exists",
      ConsoleColors::RED,
    );
    exit(1);
  }

  // Create the initial module file and write it to the file at `src/module_name`.rs.
  let mod_template = if is_plugin {
    include_str!("./templates/mod-plugin.rs")
  } else {
    include_str!("./templates/mod.rs")
  };
  let mod_template_with_module = mod_template.replace("MODULE_NAME", &module_name_pascal_case);
  write_and_fmt(
    format!("src/{}.rs", &module_name_snake_case),
    mod_template_with_module,
  )
  .expect("Unable to write initial module file to library");

  // Get the contents of the `src/lib.rs` file so we can make an AST out of it.
  let current_dir = std::env::current_dir().expect("Unable to get current directory");
  let mut lib_contents = std::fs::read_to_string(current_dir.join("src").join("lib.rs"))
    .expect("Unable to read src/lib.rs file");

  // Get the position of where to insert the mod statement for the module.
  let mod_insert_location =
    get_insert_location(project_toml.modules.clone(), &lib_contents, "mod.*;", false);

  // Insert the new module's mod line after the last module's mod line.
  let mod_line = format!("mod {};", module_name_snake_case);
  lib_contents.insert_str(mod_insert_location.0, &mod_line);

  // Next we do the same thing for the handle turbofish statement for the
  // module. However, this is different than the mod statement because if this
  // is the first module created, then we need to look for the init function.
  // If this is not the first module then we look for an existing module's
  // turbofish statement.
  let handle_insert_location = if project_toml.modules.len() == 0 {
    get_insert_location(
      project_toml.modules.clone(),
      &lib_contents,
      "init.*\\{",
      true,
    )
  } else {
    get_insert_location(
      project_toml.modules.clone(),
      &lib_contents,
      "handle.*;",
      true,
    )
  };

  // Insert the module or plugin turbofish at the start of the init function or
  // after the last module's turbofish.
  let handle_line = if is_plugin {
    format!(
      "handle.add_tool_class::<{}::{}>();",
      &module_name_snake_case, module_name_pascal_case
    )
  } else {
    format!(
      "handle.add_class::<{}::{}>();",
      &module_name_snake_case, module_name_pascal_case
    )
  };

  lib_contents.insert_str(handle_insert_location.0, &handle_line);

  // Overwrite the current contents of lib.rs with its new contents.
  write_and_fmt("src/lib.rs", lib_contents).expect("Unable to save or format");

  // Create the module's corresponding gdns file.
  let lib_name = get_library_name();
  let gdns_file_name = format!("{}.gdns", &module_name_snake_case);
  let parent_dir = current_dir.parent().expect("Unable to get parent dir");
  let gdns_path = parent_dir
    .join(&project_toml.godot_project_name)
    .join("rust_modules")
    .join(gdns_file_name);
  let gdns_template = include_str!("./templates/gdns.txt");
  let gdns_with_module_name = gdns_template
    .replace("LIBRARY_NAME", &lib_name.to_case(Case::Snake))
    .replace("MODULE_NAME", &module_name_pascal_case);
  write(gdns_path, gdns_with_module_name).expect("Unable to create module's gdns file");

  if is_plugin {
    // If the module is meant to be a plugin then we have some extra steps.
    // First we create the plugin structure within the Godot project.
    let godot_path = parent_dir.join(&project_toml.godot_project_name);
    let godot_plugin_dir = godot_path.join("addons").join(&module_name_snake_case);
    let godot_plugin_cfg = godot_plugin_dir.join("plugin.cfg");
    std::fs::create_dir_all(&godot_plugin_dir)
      .expect("Unable to create plugin directory structure in Godot project");

    // Create the config file for the plugin and replace the template values
    // with the plugin's values.
    let plugin_cfg = include_str!("./templates/plugin-cfg.txt");
    let plugin_cfg_with_name = plugin_cfg.replace("PLUGIN_NAME", &name);
    let plugin_cfg_with_script = plugin_cfg_with_name.replace(
      "PLUGIN_GDNS_LOCATION",
      &format!("../../rust_modules/{}.gdns", &module_name_snake_case),
    );
    write(godot_plugin_cfg, plugin_cfg_with_script).expect("Unable to write plugin.cfg file");
  }

  // Add the module to the project.toml
  project_toml
    .modules
    .push(module_name_pascal_case.to_string());
  set_project_toml_contents(project_toml);

  print_to_console("Module created", ConsoleColors::GREEN);
}

/// Removes a Rust module from the library.
///
/// # Arguments
///
/// `name` - The name of the module to remove.
pub fn command_destroy(name: &str) {
  print_to_console("destroying module...", ConsoleColors::WHITE);

  // Check to see if we are in the library directory as this command can only
  // be run from there.
  check_if_lib_dir();

  // Get the current modules from the project.toml.
  let mut project_toml = get_project_toml_as_object();

  let module_name_pascal_case = name.to_case(Case::Pascal);
  let module_name_snake_case = name.to_case(Case::Snake);

  // If the name of the module to destroy is not in the project.toml, then it
  // shouldn't exist and can't be destroyed.
  if !&project_toml
    .modules
    .iter()
    .any(|module| module == &module_name_pascal_case)
  {
    print_to_console("The module to destroy doesn't exist", ConsoleColors::RED);
    exit(1);
  }

  // Remove the module from the project.toml and save the changes.
  let index_of_module_to_remove = project_toml
    .modules
    .iter()
    .position(|x| *&x == &module_name_pascal_case)
    .expect("Unable get index of module to remove");
  project_toml.modules.remove(index_of_module_to_remove);

  // Get the lib file so that we can remove the module definition from it.
  let current_dir = std::env::current_dir().expect("Unable to get current directory");
  let lib_contents = std::fs::read_to_string(current_dir.join("src").join("lib.rs"))
    .expect("Unable to read src/lib.rs file");

  // Create the mod and handle strings we're searching for.
  let mod_replace = format!("mod {};", &module_name_snake_case);
  let handle_replace = format!(
    "handle.add_class::<{}::{}>();",
    &module_name_snake_case, &module_name_pascal_case
  );
  let handle_replace_plugin = format!(
    "handle.add_tool_class::<{}::{}>();",
    &module_name_snake_case, &module_name_pascal_case
  );

  // Remove the mod and handle strings from the lib file.
  let file_contents_no_mod = lib_contents
    .lines()
    .filter(|&line| line.trim() != mod_replace)
    .collect::<Vec<_>>()
    .join("\n");

  let file_contents_no_mod_no_handle = file_contents_no_mod
    .lines()
    .filter(|&line| line.trim() != handle_replace)
    .collect::<Vec<_>>()
    .join("\n");

  let file_contents_no_mod_no_handles = file_contents_no_mod_no_handle
    .lines()
    .filter(|&line| line.trim() != handle_replace_plugin)
    .collect::<Vec<_>>()
    .join("\n");

  // Now write and format the new lib file back to the disk.
  write_and_fmt("src/lib.rs", file_contents_no_mod_no_handles).expect("Unable to write lib file");

  // Deletes the module's file from the library.
  let module_file_name = format!("src/{}.rs", &module_name_snake_case);
  let module_path = Path::new(&module_file_name);
  remove_file(module_path).expect("Unable to remove module file");

  // Deletes the module's gdns file from the Godot project directory.
  let gdns_file_name = format!("{}.gdns", &module_name_snake_case);
  let parent_dir = current_dir
    .parent()
    .expect("Unable to get parent directory");
  let gdns_path = parent_dir
    .join(&project_toml.godot_project_name)
    .join("rust_modules")
    .join(gdns_file_name);
  remove_file(gdns_path).expect("Unable to remove module's gdns file");

  // Save the changes to the project.toml config file.
  set_project_toml_contents(project_toml);

  // Since the module being destroyed could be a plugin, we need to check the
  // plugin folder in the Godot project for the presence of the module.
  let godot_project_name = get_project_toml_as_object().godot_project_name;
  let plugin_path = parent_dir
    .join(&godot_project_name)
    .join("addons")
    .join(&module_name_snake_case);
  if plugin_path.exists() {
    std::fs::remove_dir_all(plugin_path)
      .expect("Unable to remove plugin directory from Godot project")
  }

  print_to_console("Module destroyed", ConsoleColors::GREEN);
}

/// Builds the library to generate the dynamic libraries needed to run the
/// modules in the Godot project.
pub fn command_build() {
  let version_notice = format!(
    "{}{}",
    "godot-rust-helper".white().underline(),
    VERSION.white().underline()
  );
  print_to_console(&version_notice, ConsoleColors::WHITE);
  print_to_console("building...", ConsoleColors::CYAN);

  let project_toml = get_project_toml_as_object();

  // Run `cargo build` to build the library.
  let cargo_build = Command::new("cargo")
    .arg("build")
    .status()
    .expect("Unable to run cargo build");
  if !cargo_build.success() {
    print_to_console("Build failed", ConsoleColors::RED);
    exit(1);
  }

  // Get the name of the library from the file name of the current directory.
  let current_dir = std::env::current_dir().expect("Unable to get current directory");
  let lib_name = get_library_name();

  // Get the path to where the dynamic libraries that were built are stored.
  let dynamic_libraries_path = get_dynamic_libraries_path();

  // Get the extention of the dynamic library generated on the os that the
  // command is being run on.
  let dynamic_library_ext = get_dynamic_library_ext();

  // If the platform that the library is being built on is not windows, then we
  // need to add an extra "lib" part before the dynamic library file name.
  let dynamic_library_extra = if cfg!(windows) { "" } else { "lib" };

  // Now we can join all of the information together to get the path to the
  // dynamic library file name.
  let dynamic_library_file_name = format!(
    "{}{}.{}",
    dynamic_library_extra,
    lib_name.to_case(Case::Snake),
    dynamic_library_ext
  );
  let dynamic_library_file = dynamic_libraries_path.join(dynamic_library_file_name);

  // Create the `bin` folder in the Godot project if it doesn't already exist.
  let parent_dir = current_dir.parent().expect("Unable to get parent dir");
  let bin_path = parent_dir
    .join(&project_toml.godot_project_name)
    .join("bin");
  std::fs::create_dir_all(&bin_path).expect("Unable to create bin directory in the Godot project");

  // Now we can copy the dynamic library file to the Godot project directory.
  Command::new("cp")
    .arg(dynamic_library_file)
    .arg(bin_path)
    .output()
    .expect("Unable to copy dynamic library to Godot project");

  print_to_console("Build complete", ConsoleColors::GREEN);
}

/// Watches the `src` directory in the library for changes and runs the build
/// command automatically.
pub fn command_build_watch() {
  let (tx, rx) = channel();

  // Run the build command for the initial build.
  build_with_timestamp();

  let mut last_checked = Local::now();
  let mut watcher = raw_watcher(tx).expect("Unable to create watcher");
  let current_dir = std::env::current_dir().expect("Unable to get current directory");
  watcher
    .watch(current_dir.join("src"), RecursiveMode::Recursive)
    .expect("Unable to watch src directory");

  loop {
    match rx.recv() {
      Ok(RawEvent {
        path: Some(_path),
        op: Ok(op),
        cookie: _,
      }) => {
        if op.contains(op::WRITE) {
          let now = Local::now();
          if (now - last_checked).num_seconds() == 0 {
            build_with_timestamp();
          }
          last_checked = Local::now();
        }
      }
      Ok(_event) => print_to_console("broken event", ConsoleColors::RED),
      Err(e) => print_to_console(&e.to_string(), ConsoleColors::RED),
    }
  }
}

/// Runs the build command and logs the time that the build was started.
fn build_with_timestamp() {
  let dt = Local::now();
  let dt_formatted = dt.format("%Y-%m-%d %H:%M:%S").to_string();

  command_build();

  print_to_console("", ConsoleColors::WHITE);
  print_to_console(
    &format!("[{}] {}", dt_formatted, "waiting for changes..."),
    ConsoleColors::WHITE,
  );
}
