use std::env::{current_dir, set_current_dir};
use std::fs::{create_dir_all, read_to_string, remove_file, write};
use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use std::sync::mpsc::channel;

use chrono::Local;
use convert_case::{Case, Casing};
use notify::{op, raw_watcher, RawEvent, RecursiveMode, Watcher};
use walkdir::WalkDir;

use crate::build_utils::build_for_platform;
use crate::config_utils::{
    add_module_to_config, create_initial_config, get_config_as_object, is_module_in_config,
    remove_module_from_config_if_exists,
};
use crate::definitions::CargoToml;
use crate::file_utils::write_and_fmt;
use crate::gdnlib_utils::create_initial_gdnlib;
use crate::lib_utils::add_module_to_lib;
use crate::log_utils::{
    log_error_to_console, log_info_to_console, log_styled_message_to_console,
    log_success_to_console, log_version, ConsoleColors,
};
use crate::path_utils::{exit_if_not_lib_dir, get_absolute_path};

/// Creates the library used to manage Rust modules.
///
/// # Arguments
///
/// `name` - The name of the library.
/// `godot_project_dir` - The relative path to the directory of the Godot project the plugin or modules are for.
/// `plugin` - Indicates whether the library is for a plugin or not.
/// `skip_build` - Indicates whether the build should be skipped after creating the library or not.
pub fn command_new(name: &str, godot_project_dir: PathBuf, plugin: bool, skip_build: bool) {
    log_styled_message_to_console("Creating library", ConsoleColors::WHITE);

    let library_name_normalized = name.to_case(Case::Snake);

    let library_absolute_path = get_absolute_path(&PathBuf::from(&name));
    let godot_project_absolute_path = get_absolute_path(&godot_project_dir);

    // If there's already a directory with the library name then we print an
    // error to the console and exit early.
    if library_absolute_path.exists() {
        log_styled_message_to_console(
            "Cannot create library, directory with the same name already exists",
            ConsoleColors::RED,
        );
        exit(1);
    }

    // If there's not a project.godot file at the root of the provided Godot
    // project directory then we print an error to the console and exit early.
    if !godot_project_absolute_path.join("project.godot").exists() {
        log_styled_message_to_console(
            "The Godot project dir provided is not valid",
            ConsoleColors::RED,
        );
        exit(1);
    }

    // Creates the library using the `cargo new --lib` command.
    match Command::new("cargo")
        .arg("new")
        .arg(&library_name_normalized)
        .arg("--lib")
        .output()
    {
        Ok(_v) => (),
        Err(e) => {
            log_styled_message_to_console(&e.to_string(), ConsoleColors::RED);
            exit(1);
        }
    }

    set_current_dir(&library_name_normalized).expect("Unable to change to library directory");

    // Get the base Cargo.toml contents of the library.
    let library_cargo_toml_string = read_to_string("Cargo.toml")
        .expect("Unable to read library's Cargo.toml file while creating the library");

    // Add the necessary dependencies to the base contents.
    let new_library_cargo_toml: CargoToml = toml::from_str(&library_cargo_toml_string)
        .expect("Unable to parse the library's Cargo.toml file");

    // Turn the new contents of the library's Cargo.toml into a string so that we
    // can write it back to the library. We also need to normalize some things here
    // because when we turn the Cargo toml contents to a string, extra symbols get
    // added.
    let new_library_cargo_toml_string = toml::to_string(&new_library_cargo_toml)
        .expect(
            "Unable to convert the library's new Cargo.toml to a string while creating the library",
        )
        .replace("\\", "")
        .replace("\"{", "{")
        .replace("}\"", "}");

    // Next we overwrite the contents of the Cargo.toml file with our contents.
    write("Cargo.toml", new_library_cargo_toml_string).expect(
        "Unable to update contents of the library's Cargo.toml file while creating the library",
    );

    let godot_project_dir_name = godot_project_absolute_path
        .file_name()
        .unwrap()
        .to_str()
        .expect("Unable to convert Godot file name to str")
        .to_string();
    let config = create_initial_config(name.to_owned(), godot_project_dir_name, plugin);

    // Creates the initial `lib.rs` file in the library directory.
    let lib_template = include_str!("./templates/lib.rs");
    write_and_fmt("src/lib.rs", lib_template).expect(
        "Unable to create the initial lib.rs file in the library while creating the library",
    );

    log_styled_message_to_console(
        "running initial build to generate Godot project structure",
        ConsoleColors::CYAN,
    );

    if plugin {
        let module_name_snake_case = &name.to_case(Case::Snake);

        let godot_plugin_dir = godot_project_absolute_path
            .join("addons")
            .join(&module_name_snake_case);
        let godot_plugin_cfg = godot_plugin_dir.join("plugin.cfg");
        create_dir_all(&godot_plugin_dir)
        .expect("Unable to create the plugin directory structure in Godot project while creating the library");

        command_create(&name);

        let plugin_cfg = include_str!("./templates/plugin-cfg.txt");
        let plugin_cfg_with_name = plugin_cfg.replace("PLUGIN_NAME", &name);
        let plugin_cfg_with_script = plugin_cfg_with_name.replace(
            "PLUGIN_GDNS_LOCATION",
            &format!("{}.gdns", &module_name_snake_case),
        );
        write(godot_plugin_cfg, plugin_cfg_with_script).expect(
            "Unable to write plugin.cfg file in the Godot project while creating the library",
        );
    }

    // Creates the gdnative directory within the Godot project.
    let gdnative_path = if config.is_plugin {
        godot_project_absolute_path
            .join("addons")
            .join(&library_name_normalized)
            .join("gdnative")
    } else {
        godot_project_absolute_path.join("gdnative")
    };

    match create_dir_all(&gdnative_path) {
        Ok(_) => (),
        Err(e) => {
            // If there was a problem creating the directory then we print the error
            // to the console and exit early.
            log_styled_message_to_console(&e.to_string(), ConsoleColors::RED);
            exit(1);
        }
    }

    create_initial_gdnlib(&config);

    // For testing we skip building the library so that tests won't take a
    // long time to run. We already test building on its own so it isn't
    // necessary to run here.
    if !skip_build {
        // Otherwise, in normal environments, we want to run the initial build
        // or else Godot will throw errors stating it can't find the dynamic
        // library for the project.
        command_build(false, false);
    }

    log_styled_message_to_console("library created", ConsoleColors::GREEN);
}

/// Creates a module by creating a module for it inside the library and a
/// corresponding gdns file in the Godot project.
///
/// # Arguments
///
/// `name` - The name of the module to create as pascal case.
pub fn command_create(name: &str) {
    exit_if_not_lib_dir();

    let module_name_snake_case = &name.to_case(Case::Snake);
    let module_name_pascal_case = &name.to_case(Case::Pascal);

    let current_dir_path =
        current_dir().expect("Unable to get current directory while creating the module");
    let parent_dir_path = current_dir_path
        .parent()
        .expect("Unable to get the shared directory while creating the module");

    let mut config = get_config_as_object();

    let path_to_godot_project = parent_dir_path.join(&config.godot_project_dir_name);

    log_styled_message_to_console("Creating module", ConsoleColors::WHITE);

    if is_module_in_config(name, &mut config) {
        // If there's already a module with the same name in the config, then
        // we exist early to avoid creating duplicates.
        log_styled_message_to_console(
            "A module with the same name already exists",
            ConsoleColors::RED,
        );
    }

    // Creates the initial file for the module in the library directory.
    // Get the template for the module depending whether it's a regular module
    // or a plugin module.
    let mod_template = if config.is_plugin {
        include_str!("./templates/mod-plugin.rs")
    } else {
        include_str!("./templates/mod.rs")
    };

    // Replace the values in the default module template with the pascal
    // version of the module name and write the file to the library's src
    // directory.
    let mod_template_with_module = mod_template.replace("MODULE_NAME", &module_name_pascal_case);
    write_and_fmt(
        format!("src/{}.rs", &module_name_snake_case),
        mod_template_with_module,
    )
    .expect("Unable to create the initial module file in the library while creating a module");

    add_module_to_lib(name, &config);

    // Creates the gdns file for the module from the template and places it either
    // in the gdnative directory at the root of the Godot project if it is a
    // normal library or in the gdnative directory at the root of the plugin
    // directory in the Godot project if it is a plugin library.
    let gdns_file_name = format!("{}.gdns", &module_name_snake_case);
    let library_name_snake_case = &config.name.to_case(Case::Snake);

    let gdns_dir: PathBuf = if config.is_plugin {
        path_to_godot_project
            .join("addons")
            .join(&library_name_snake_case)
            .join("gdnative")
    } else {
        path_to_godot_project.join("gdnative")
    };

    create_dir_all(&gdns_dir).expect("Unable to create directory for module file in Godot.");

    // The path to the gdnlib file in the Godot project.
    let gdnlib_path = if config.is_plugin {
        format!(
            "addons/{}/gdnative/{}",
            &library_name_snake_case, &library_name_snake_case
        )
    } else {
        format!("gdnative/{}", &library_name_snake_case)
    };

    // Replace the values in our template with the name of the library and the
    // pascal version of the module name.
    let gdns_template = include_str!("./templates/gdns.txt");
    let gdns_with_module_name = gdns_template
        .replace("GDNLIB_PATH", &gdnlib_path)
        .replace("MODULE_NAME", &module_name_pascal_case);

    write(gdns_dir.join(&gdns_file_name), gdns_with_module_name)
        .expect("Unable to create module's gdns file while creating the module");

    add_module_to_config(name, &mut config);

    log_styled_message_to_console("Module created", ConsoleColors::GREEN);
}

/// Removes a module by deleting its module file from the library and searching
/// the Godot project for the corresponding gdns file to remove.
///
/// # Arguments
///
/// `name` - The name of the module to remove.
pub fn command_destroy(name: &str) {
    exit_if_not_lib_dir();

    log_styled_message_to_console("destroying module...", ConsoleColors::WHITE);

    let mut config = get_config_as_object();

    let module_name_snake_case = name.to_case(Case::Snake);
    let module_name_pascal_case = name.to_case(Case::Pascal);

    let current_dir_path =
        current_dir().expect("Unable to get current directory while destroying the module");
    let parent_dir = current_dir_path
        .parent()
        .expect("Unable to get shared directory while destroying the module");
    let path_to_godot_project = parent_dir.join(&config.godot_project_dir_name);

    remove_module_from_config_if_exists(&module_name_pascal_case, &mut config);

    // Removes the module's gdns file from Godot project.
    let library_name_snake_case = &config.name.to_case(Case::Snake);
    let gdns_file_name = format!("{}.gdns", &module_name_snake_case);

    // The first place we should check for the module to remove is either the
    // gdnative folder in the plugin directory if it's a plugin or just the
    // gdnative folder in the root directory of the Godot project otherwise.
    let possible_gdns_path = if config.is_plugin {
        path_to_godot_project
            .join("addons")
            .join(&library_name_snake_case)
            .join("gdnative")
            .join(&gdns_file_name)
    } else {
        path_to_godot_project.join("gdnative").join(&gdns_file_name)
    };

    if possible_gdns_path.exists() {
        // If this path exists, then we can just remove the module.
        remove_file(possible_gdns_path).expect("Unable to remove the module's gdns file from the Godot project while destroying the module");
    } else {
        // Otherwise, we want to search a directory for the module. If the
        // module is a plugin, we can limit our search to the plugin directory.
        // Otherwise, we search the entire project since the user might have
        // moved it around.
        let search_dir = if config.is_plugin {
            path_to_godot_project
                .join("addons")
                .join(&library_name_snake_case)
        } else {
            path_to_godot_project.to_owned()
        };

        for entry in WalkDir::new(search_dir).into_iter().filter_map(|e| e.ok()) {
            let file_name = entry
                .file_name()
                .to_str()
                .expect("Unable to get file name while finding module to remove in Godot project");
            if file_name == gdns_file_name {
                remove_file(entry.path()).expect("Unable to remove module's gdns file");
            }
        }
    }

    // Removes all traces of a module from the lib.rs file.
    let lib_file_contents = read_to_string(current_dir_path.join("src").join("lib.rs"))
        .expect("Unable to read the contents of the lib file while destroying the module");

    // Create the mod and handle strings that we want to search for and remove
    // from the lib file.
    let module_mod_search_query = format!("mod {};", &module_name_snake_case);
    let module_handle_search_query = format!(
        "handle.add_class::<{}::{}>();",
        &module_name_snake_case, &module_name_pascal_case
    );
    let module_plugin_handle_search_query = format!(
        "handle.add_tool_class::<{}::{}>();",
        &module_name_snake_case, &module_name_pascal_case
    );

    // Remove the `mod` declaration for the module.
    let file_contents_no_mod = lib_file_contents
        .lines()
        .filter(|&line| line.trim() != module_mod_search_query)
        .collect::<Vec<_>>()
        .join("\n");

    // Removes the turbofish handle for a module in a normal library.
    let file_contents_no_mod_no_handle = file_contents_no_mod
        .lines()
        .filter(|&line| line.trim() != module_handle_search_query)
        .collect::<Vec<_>>()
        .join("\n");

    // Removes the turbofish handle for a module in a plugin library.
    let file_contents_no_mod_no_handles = file_contents_no_mod_no_handle
        .lines()
        .filter(|&line| line.trim() != module_plugin_handle_search_query)
        .collect::<Vec<_>>()
        .join("\n");

    write_and_fmt("src/lib.rs", file_contents_no_mod_no_handles)
        .expect("Unable to write the new contents to the lib.rs file while destroying the module");

    // Removes the module's file from the library.
    let module_file_name = format!("src/{}.rs", &module_name_snake_case);
    let module_file_path = Path::new(&module_file_name);
    remove_file(module_file_path)
        .expect("Unable to remove the module file from the library while destroying the module");

    log_styled_message_to_console("Module destroyed", ConsoleColors::GREEN);
}

/// Runs the command to build the library and then copies over the dynamic
/// libraries to the Godot project.
///
/// `is_release` - Indicates whether the build is a release build or not.
/// `build_all_platforms` - Indicates whether all platforms should be built or just the native one.
pub fn command_build(is_release: bool, build_all_platforms: bool) {
    log_version();
    log_info_to_console("[build] build starting...");

    let current_dir = std::env::current_dir().expect("[build] Unable to get current directory.");
    let parent_dir = current_dir
        .parent()
        .expect("[build] Unable to get parent directory.");

    let config = get_config_as_object();
    let library_name_snake_case = &config.name.to_case(Case::Snake);

    // Build for the native platform by default.
    let native_platform = std::env::consts::OS.to_lowercase();
    build_for_platform(
        parent_dir,
        &config,
        &library_name_snake_case,
        &native_platform,
        is_release,
    );

    // Build for all platforms if the flag is passed.
    if build_all_platforms {
        for platform in &config.platforms {
            build_for_platform(
                parent_dir,
                &config,
                &library_name_snake_case,
                &platform.to_lowercase(),
                is_release,
            );
        }
    }

    // Let the user know that the build is complete.
    log_success_to_console("[build] build complete");
}

/// Watches the src directory in the library for changes and rebuilds the
/// library when changes are detected.
///
/// # Arguments
///
/// `is_release` - Indicates whether the build is a release build or not.
/// `build_all_targets` - Indicates whether all of the targets should be built instead of just the native target.
pub fn command_build_and_watch(is_release: bool, build_all_targets: bool) {
    let (tx, rx) = channel();

    build_library_with_timestamp(is_release, build_all_targets);

    let mut last_checked = Local::now();
    let mut watcher =
        raw_watcher(tx).expect("Unable to create watcher to watch library for changes");
    let current_dir = std::env::current_dir()
        .expect("Unable to get current directory while attempting to watch library for changes");

    watcher
        .watch(current_dir.join("src"), RecursiveMode::Recursive)
        .expect("Unable to watch library directory for changes");
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
                        build_library_with_timestamp(is_release, build_all_targets);
                    }
                    last_checked = Local::now();
                }
            }
            Ok(_event) => log_error_to_console("broken event"),
            Err(e) => log_error_to_console(&e.to_string()),
        }
    }
}

/// Runs the `build_library` function to build the library and copy the
/// dynamic library file to the Godot project.
///
/// In addition to that, it also logs the datetime that the build was
/// completed as `YYYY-MM-DD HH:MM::SS and lets the user know that it is
/// waiting for changes before building again.
///
/// # Arguments
///
/// `is_release` - Indicates whether the build is a release build or not.
/// `build_all_targets` - Indicates whether all of the targets should be built instead of just the native target.
pub fn build_library_with_timestamp(is_release: bool, build_all_targets: bool) {
    command_build(is_release, build_all_targets);

    let dt = Local::now();
    let current_datetime_formatted = dt.format("%Y-%m-%d %H:%M:%S").to_string();

    // After the build we want to show a message letting the user know that the
    // build has finished and is waiting for futher changes before rebuilding.
    log_info_to_console("");
    log_info_to_console(&format!(
        "[{}] {}",
        current_datetime_formatted, "waiting for changes..."
    ));
}
