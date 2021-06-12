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

/// Creates a new Rust module inside the library.
///
/// # Arguments
///
/// `name` - The name of the module to create. The module name should be PascalCase with examples including 'Player', 'Princess', 'Mob', etc.
pub fn create_module(name: &str) {
    // Exit early if this command is not being run from the library directory.
    exit_if_not_lib_dir();

    // We need the name of the module to create as snake case for file names
    // and as pascal case for config.
    let module_name_snake_case = &name.to_case(Case::Snake);
    let module_name_pascal_case = &name.to_case(Case::Pascal);

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
    create_initial_module_file(
        module_name_snake_case,
        module_name_pascal_case,
        config.is_plugin,
    );

    // Add the `mod` and turbofish handle to the `src/lib.rs` file in the
    // library directory.
    add_module_to_lib(name, &config);

    // Create the gdns file for the module in the Godot project directory.
    // Note that this puts the gdns file in the `rust_modules` directory by
    // default but since the modules reference the gdnlib file in the root
    // of the Godot project, it can be moved anywhere.
    create_gdns_file_in_godot(
        module_name_snake_case,
        module_name_pascal_case,
        &path_to_godot_project,
        &config,
    );

    // Adds the module to the `modules` section of the config and saves it.
    add_module_to_config(name, &mut config);

    log_styled_message_to_console("Module created", ConsoleColors::GREEN);
}

/// Creates the initial module file with the template.
///
/// # Arguments
///
/// `module_name_snake_case` - The snake case version of the name of the module to create.
/// `module_name_pascal_case` - The pascal case version of the name of the module to create.
/// `is_plugin` - Indicates whether the module is a plugin or not.
fn create_initial_module_file(
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
    .expect("Unable to write initial module file to library");
}

/// Creates the gdns file for the module and writes it to the Godot project
/// directory.
///
/// `module_name_snake_case` - The snake case version of the name of the module to create.
/// `module_name_pascal_case` - The pascal case version of the name of the module to create.
/// `godot_project_dir` - The path to the Godot project.
/// `is_plugin` - Indicates whether the module is for a plugin for not.
fn create_gdns_file_in_godot(
    module_name_snake_case: &str,
    module_name_pascal_case: &str,
    godot_project_dir: &PathBuf,
    config: &Config,
) {
    // Create the path to the gdns file in the Godot project by joining the
    // path to the Godot project with the default `rust_modules` directory.
    let gdns_file_name = format!("{}.gdns", &module_name_snake_case);
    let library_name_snake_dir = &config.name.to_case(Case::Snake);

    let gdns_path: PathBuf = if config.is_plugin {
        // Since the library is a plugin, we check to first check whether the
        // `rust_modules` directory exists in the Godot project or not. If it
        // does then we write the gdns file that, otherwise we write it to the
        // root of the plugin directory.
        let plugin_rust_modules_dir = godot_project_dir
            .join("addons")
            .join(&library_name_snake_dir)
            .join("rust_modules");
        if plugin_rust_modules_dir.exists() {
            plugin_rust_modules_dir.join(gdns_file_name)
        } else {
            godot_project_dir
                .join("addons")
                .join(&library_name_snake_dir)
                .join(gdns_file_name)
        }
    } else {
        godot_project_dir.join("rust_modules").join(gdns_file_name)
    };

    let gdns_library_path = if config.is_plugin {
        format!(
            "addons/{}/{}",
            &library_name_snake_dir, &library_name_snake_dir
        )
    } else {
        format!("{}", &library_name_snake_dir)
    };

    let gdns_template = include_str!("../templates/gdns.txt");
    let gdns_with_module_name = gdns_template
        .replace("LIBRARY_PATH", &gdns_library_path)
        .replace("MODULE_NAME", &module_name_pascal_case);

    write(gdns_path, gdns_with_module_name).expect("Unable to create module's gdns file");
}
