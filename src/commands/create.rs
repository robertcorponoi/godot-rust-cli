use std::env::current_dir;
use std::fs::write;
use std::path::PathBuf;

use convert_case::{Case, Casing};

use crate::config_utils::{
    add_module_to_config, get_config_as_object, is_module_in_config, Config,
};
use crate::file_utils::write_and_fmt;
use crate::lib_utils::add_module_to_lib;
use crate::log_utils::{log_styled_message_to_console, ConsoleColors};
use crate::path_utils::exit_if_not_lib_dir;

/// Creates a module by creating a module for it inside the library and a
/// corresponding gdns file in the Godot project.
///
/// # Arguments
///
/// `name` - The name of the module to create as pascal case.
pub fn create_module(name: &str) {
    exit_if_not_lib_dir();

    let module_name_snake_case = &name.to_case(Case::Snake);
    let module_name_pascal_case = &name.to_case(Case::Pascal);

    let current_dir_path =
        current_dir().expect("Unable to get current directory while creating the module");
    let parent_dir_path = current_dir_path
        .parent()
        .expect("Unable to get the shared directory while creating the module");

    let mut config = get_config_as_object();

    let path_to_godot_project = parent_dir_path.join(&config.godot_project_name);

    log_styled_message_to_console("Creating module", ConsoleColors::WHITE);

    if is_module_in_config(name, &mut config) {
        // If there's already a module with the same name in the config, then
        // we exist early to avoid creating duplicates.
        log_styled_message_to_console(
            "A module with the same name already exists",
            ConsoleColors::RED,
        );
    }

    create_initial_module_file_in_library(
        module_name_snake_case,
        module_name_pascal_case,
        config.is_plugin,
    );

    add_module_to_lib(name, &config);

    create_gdns_file_in_godot(
        module_name_snake_case,
        module_name_pascal_case,
        &path_to_godot_project,
        &config,
    );

    add_module_to_config(name, &mut config);

    log_styled_message_to_console("Module created", ConsoleColors::GREEN);
}

/// Creates the initial file for the module in the library directory.
///
/// # Arguments
///
/// `module_name_snake_case` - The snake case version of the module name.
/// `module_name_pascal_case` - The pascal case version of the module name.
/// `is_plugin` - Indicates whether the module is for plugin library or not.
fn create_initial_module_file_in_library(
    module_name_snake_case: &str,
    module_name_pascal_case: &str,
    is_plugin: bool,
) {
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
    .expect("Unable to create the initial module file in the library while creating a module");
}

/// Creates the gdns file for the module from the template and places it either
/// in the rust_modules directory at the root of the Godot project if it is a
/// normal library or in the rust_modules directory at the root of the plugin
/// directory in the Godot project if it is a plugin library.
///
/// # Arguments
///
/// `module_name_snake_case` - The snake case version of the module name.
/// `module_name_pascal_case` - The pascal case version of the module name.
/// `godot_project_path` - The absolute path to the Godot project.
/// `config` - The current configuration object.
fn create_gdns_file_in_godot(
    module_name_snake_case: &str,
    module_name_pascal_case: &str,
    godot_project_dir: &PathBuf,
    config: &Config,
) {
    let gdns_file_name = format!("{}.gdns", &module_name_snake_case);
    let library_name_snake_case = &config.name.to_case(Case::Snake);

    let gdns_path: PathBuf = if config.is_plugin {
        // Since the library is a plugin, we check to first check whether the
        // `rust_modules` directory exists in the Godot project or not. If it
        // does then we write the gdns file that, otherwise we write it to the
        // root of the plugin directory.
        let plugin_rust_modules_dir = godot_project_dir
            .join("addons")
            .join(&library_name_snake_case)
            .join("rust_modules");
        if plugin_rust_modules_dir.exists() {
            plugin_rust_modules_dir.join(gdns_file_name)
        } else {
            godot_project_dir
                .join("addons")
                .join(&library_name_snake_case)
                .join(gdns_file_name)
        }
    } else {
        godot_project_dir.join("rust_modules").join(gdns_file_name)
    };

    // The path to the library is either the root directory of the plugin if it
    // is a plugin or just the root of the Godot project otherwise.
    let gdns_library_path = if config.is_plugin {
        format!(
            "addons/{}/{}",
            &library_name_snake_case, &library_name_snake_case
        )
    } else {
        format!("{}", &library_name_snake_case)
    };

    // Replace the values in our template with the name of the library and the
    // pascal version of the module name.
    let gdns_template = include_str!("../templates/gdns.txt");
    let gdns_with_module_name = gdns_template
        .replace("LIBRARY_PATH", &gdns_library_path)
        .replace("MODULE_NAME", &module_name_pascal_case);

    write(gdns_path, gdns_with_module_name)
        .expect("Unable to create module's gdns file while creating the module");
}
