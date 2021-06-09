use std::env::current_dir;
use std::fs::write;
use std::path::PathBuf;

use convert_case::{Case, Casing};

use crate::config_utils::{add_module_to_config, get_config_as_object, is_module_in_config};
use crate::file_utils::write_and_fmt;
use crate::lib_utils::add_module_to_lib;
use crate::log_utils::{log_styled_message_to_console, ConsoleColors};
use crate::path_utils::{exit_if_not_lib_dir, get_library_name_from_path};

/// Creates a new Rust module inside the library.
///
/// # Arguments
///
/// `name` - The name of the module to create. The module name should be PascalCase with examples including 'Player', 'Princess', 'Mob', etc.
/// `is_plugin` - Indicates whether the module is for a plugin or not.
pub fn create_module(name: &str, is_plugin: bool) {
    // Exit early if this command is not being run from the library directory.
    exit_if_not_lib_dir();

    // Get the parent directory since it always contains both the library and Godot project.
    let current_dir = current_dir().expect("Unable to get current directory");
    let parent_dir = current_dir.parent().expect("Unable to get parent dir");

    // The mutable contents of the configuration file.
    let mut config = get_config_as_object();

    // The path to the Godot project.
    let path_to_godot_project = parent_dir.join(&config.godot_project_name);

    log_styled_message_to_console("Creating module", ConsoleColors::WHITE);

    if is_module_in_config(name, &mut config) {
        // If there's already a module with the same name in the config, then
        // we exist early to avoid creating duplicates.
        log_styled_message_to_console(
            "A module with the same name already exists",
            ConsoleColors::RED,
        );
    }
    // Create the `module.mod` file for the module in the library directory.
    create_initial_module_file(name, is_plugin);

    // Add the `mod` and turbofish handle to the `src/lib.rs` file in the
    // library directory.
    add_module_to_lib(name, is_plugin, &config);

    // Create the gdns file for the module in the Godot project directory.
    // Note that this puts the gdns file in the `rust_modules` directory by
    // default but since the modules reference the gdnlib file in the root
    // of the Godot project, it can be moved anywhere.
    create_gdns_file_in_godot_project(name, &config.godot_project_name);

    if is_plugin {
        // If this module is a plugin, then we need to create the plugin
        // structure in the Godot project.
        create_plugin_structure_in_godot_project(name, &path_to_godot_project)
    }

    // Adds the module to the `modules` section of the config and saves it.
    add_module_to_config(name, &mut config);

    log_styled_message_to_console("Module created", ConsoleColors::GREEN);
}

/// Creates the plugin structure within the Godot project.
///
/// `godot_project_path` - The path to the Godot project.
/// `module_name` - The name of the plugin module to create.
/// `normalized_module_name` - The normalized name of the plugin module to create.
fn create_plugin_structure_in_godot_project(module_name: &str, godot_project_path: &PathBuf) {
    // Normalize the module name because we want directories and files in the
    // Godot project to be lowercase due to standards.
    let module_name_snake_case = &module_name.to_case(Case::Snake);

    // Create the directory for the plugin and the necessary `plugin.cfg` file
    // required by Godot.
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

/// Creates the initial module file with the template.
///
/// # Arguments
///
/// `module_name` - The name of the module to create.
/// `is_plugin` - Indicates whether the module is a plugin or not.
fn create_initial_module_file(module_name: &str, is_plugin: bool) {
    // We need two variations of the module name to create the mod file, the
    // snake case version and the pascal case version. The snake case version
    // is used to create the module's file name while the pascal case version
    // is used in the module template file.
    let module_name_snake_case = &module_name.to_case(Case::Snake);
    let module_name_pascal_case = &module_name.to_case(Case::Pascal);

    // Get the template for the module depending whether it's a regular module
    // or a plugin module.
    let mod_template = if is_plugin {
        include_str!("../templates/mod-plugin.rs")
    } else {
        include_str!("../templates/mod.rs")
    };

    // Replace the values in the default module template with the pascal
    // version of the module name and write the file to the library's src
    // directory.
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
    // We need two versions of the module name, the snake case version for the
    // gdns file name and the pascal version for replacing the module's name in
    // the gdns file template.
    let module_name_snake_case = &module_name.to_case(Case::Snake);
    let module_name_pascal_case = &module_name.to_case(Case::Pascal);

    // Get the name of the library.
    let lib_name = get_library_name_from_path();

    // Get the parent directory so that we can create paths to the Godot
    // project since the parent directory contains both the library and the
    // Godot project.
    let current_dir = current_dir().expect("Unable to get current directory");
    let parent_dir = current_dir.parent().expect("Unable to get parent dir");

    // Create the path to the gdns file in the Godot project by joining the
    // path to the Godot project with the default `rust_modules` directory.
    let gdns_file_name = format!("{}.gdns", &module_name_snake_case);
    let gdns_path = parent_dir
        .join(&godot_project_name)
        .join("rust_modules")
        .join(gdns_file_name);

    // Get the template for the gdns file and replace the values with the
    // module name.
    let gdns_template = include_str!("../templates/gdns.txt");
    let gdns_with_module_name = gdns_template
        .replace("LIBRARY_NAME", &lib_name.to_case(Case::Snake))
        .replace("MODULE_NAME", &module_name_pascal_case);

    write(gdns_path, gdns_with_module_name).expect("Unable to create module's gdns file");
}
