use std::env::current_dir;
use std::fs::{read_to_string, write};
use std::path::PathBuf;
use std::process::exit;

use convert_case::{Case, Casing};

use crate::config_utils::{add_module_to_config, get_config_as_object, is_module_in_config};
use crate::file_utils::{get_insert_location, write_and_fmt};
use crate::log_utils::{log_styled_message_to_console, ConsoleColors};
use crate::path_utils::{exit_if_not_lib_dir, get_library_name_from_path};

/// Creates a new Rust module inside the library.
///
/// # Arguments
///
/// `name`      - The name of the module to create. The module name should be PascalCase with examples including 'Player', 'Princess', 'Mob', etc.
/// `is_plugin` - Indicates whether the module is for a plugin or not.
pub fn create_module(name: &str, is_plugin: bool) {
    let current_dir = current_dir().expect("Unable to get current directory");
    let parent_dir = current_dir.parent().expect("Unable to get parent dir");

    // The mutable contents of the configuration file.
    let config = get_config_as_object();

    // The path to the Godot project.
    let godot_path = parent_dir.join(&config.godot_project_name);

    log_styled_message_to_console("Creating module", ConsoleColors::WHITE);

    exit_if_not_lib_dir();

    check_if_module_already_exists(name, &config.modules);
    create_initial_module_file(name, is_plugin);

    add_module_to_lib(name, &config.modules, is_plugin);

    // Create the module's corresponding gdns file.
    create_gdns_file_in_godot_project(name, &config.godot_project_name);

    if is_plugin {
        create_plugin_structure_in_godot_project(name, godot_path)
    }

    add_module_to_config(name);
    log_styled_message_to_console("Module created", ConsoleColors::GREEN);
}

/// Creates the plugin structure within the Godot project.
///
/// `godot_project_path` - The path to the Godot project.
/// `module_name` - The name of the plugin module to create.
/// `normalized_module_name` - The normalized name of the plugin module to create.
fn create_plugin_structure_in_godot_project(module_name: &str, godot_project_path: PathBuf) {
    let module_name_snake_case = &module_name.to_case(Case::Snake);

    let godot_plugin_dir = godot_project_path
        .join("addons")
        .join(&module_name_snake_case);
    let godot_plugin_cfg = godot_plugin_dir.join("plugin.cfg");

    std::fs::create_dir_all(&godot_plugin_dir)
        .expect("Unable to create plugin directory structure in Godot project");

    // Create the config file for the plugin and replace the template values
    // with the plugin's values.
    let plugin_cfg = include_str!("../templates/plugin-cfg.txt");
    let plugin_cfg_with_name = plugin_cfg.replace("PLUGIN_NAME", &module_name);
    let plugin_cfg_with_script = plugin_cfg_with_name.replace(
        "PLUGIN_GDNS_LOCATION",
        &format!("../../rust_modules/{}.gdns", &module_name_snake_case),
    );

    write(godot_plugin_cfg, plugin_cfg_with_script).expect("Unable to write plugin.cfg file");
}

/// Checks to see if the module with the provided name already exists and if it
/// does, we exit early.
///
/// # Arguments
///
/// `module_name` - The name of the module to create.
/// `config_modules` - The modules of the config file.
fn check_if_module_already_exists(module_name: &str, config_modules: &Vec<String>) {
    let module_name_pascal_case = &module_name.to_case(Case::Pascal);

    let module_exists = is_module_in_config(&config_modules, &module_name_pascal_case);
    if module_exists {
        log_styled_message_to_console(
            "A module with the same name already exists",
            ConsoleColors::RED,
        );
        exit(1);
    }
}

/// Creates the initial module file with the template.
///
/// # Arguments
///
/// `module_name` - The name of the module to create.
/// `is_plugin` - Indicates whether the module is a plugin or not.
fn create_initial_module_file(module_name: &str, is_plugin: bool) {
    let module_name_snake_case = &module_name.to_case(Case::Snake);
    let module_name_pascal_case = &module_name.to_case(Case::Pascal);

    let mod_template = if is_plugin {
        include_str!("../templates/mod-plugin.rs")
    } else {
        include_str!("../templates/mod.rs")
    };
    let mod_template_with_module = mod_template.replace("MODULE_NAME", &module_name_pascal_case);
    write_and_fmt(
        format!("src/{}.rs", &module_name_snake_case),
        mod_template_with_module,
    )
    .expect("Unable to write initial module file to library");
}

/// Creates the gdns file for the module and writes it to the Godot project
/// directory.
///
/// `module_name` - The name of the module to create.
/// `godot_project_name` - The name of the project.
/// `parent_dir` - The parent directory.
/// `module_name_snake_case` - The snake_case version of the module name.
/// `module_name_pascal_case` - The PascalCase version of the module to check if already exists.
fn create_gdns_file_in_godot_project(module_name: &str, godot_project_name: &String) {
    let module_name_snake_case = &module_name.to_case(Case::Snake);
    let module_name_pascal_case = &module_name.to_case(Case::Pascal);

    let lib_name = get_library_name_from_path();

    let current_dir = current_dir().expect("Unable to get current directory");
    let parent_dir = current_dir.parent().expect("Unable to get parent dir");

    let gdns_file_name = format!("{}.gdns", &module_name_snake_case);
    let gdns_path = parent_dir
        .join(&godot_project_name)
        .join("rust_modules")
        .join(gdns_file_name);

    let gdns_template = include_str!("../templates/gdns.txt");
    let gdns_with_module_name = gdns_template
        .replace("LIBRARY_NAME", &lib_name.to_case(Case::Snake))
        .replace("MODULE_NAME", &module_name_pascal_case);

    write(gdns_path, gdns_with_module_name).expect("Unable to create module's gdns file");
}

/// Adds the module to the library's `lib.rs` file.
///
/// `module_name` - The name of the module.
/// `config_modules` - The modules from the config file.
/// `is_plugin` - Indicates whether the module is a plugin or not.
fn add_module_to_lib(module_name: &str, config_modules: &Vec<String>, is_plugin: bool) {
    let module_name_snake_case = &module_name.to_case(Case::Snake);
    let module_name_pascal_case = &module_name.to_case(Case::Pascal);

    let current_dir = current_dir().expect("Unable to get current directory");

    let mut lib_contents = read_to_string(current_dir.join("src").join("lib.rs"))
        .expect("Unable to read src/lib.rs file");

    // Get the position of where to insert the mod statement for the module.
    let mod_insert_location =
        get_insert_location(&config_modules.clone(), &lib_contents, "mod.*;", false);

    // Insert the new module's mod line after the last module's mod line.
    let mod_line = format!("mod {};", module_name_snake_case);
    lib_contents.insert_str(mod_insert_location.0, &mod_line);

    // Next we do the same thing for the handle turbofish statement for the
    // module. However, this is different than the mod statement because if this
    // is the first module created, then we need to look for the init function.
    // If this is not the first module then we look for an existing module's
    // turbofish statement.
    let handle_insert_location = if config_modules.len() == 0 {
        get_insert_location(&config_modules.clone(), &lib_contents, "init.*\\{", true)
    } else {
        get_insert_location(&config_modules.clone(), &lib_contents, "handle.*;", true)
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
}
