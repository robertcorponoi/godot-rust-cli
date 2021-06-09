use std::fs::remove_file;
use std::path::Path;

use convert_case::{Case, Casing};

use crate::config_utils::{get_config_as_object, remove_module_from_config_if_exists};
use crate::file_utils::write_and_fmt;
use crate::log_utils::{log_styled_message_to_console, ConsoleColors};
use crate::path_utils::exit_if_not_lib_dir;

/// Removes a Rust module from the library.
///
/// # Arguments
///
/// `name` - The name of the module to remove.
pub fn destroy_module(name: &str) {
    // Exit early if this command is not being run from the library directory.
    exit_if_not_lib_dir();

    log_styled_message_to_console("destroying module...", ConsoleColors::WHITE);

    // The library configuration.
    let mut config = get_config_as_object();

    // Remove the module from the config if it exists. If it doesn't exist,
    // then an error is thrown and we return early since there is nothing to
    // remove.
    remove_module_from_config_if_exists(name, &mut config);

    // Remove the parts of the module from the Godot project. This includes the
    // gdns file for the module and the plugin directory if the module was a
    // plugin.
    remove_module_gdns_from_godot(name, config.godot_project_name);
    remove_godot_plugin_dir_if_exists(name);

    // Remove the parts of the module from the library directory. This includes
    // the module's file and it's references from the `lib.rs` file.
    remove_module_from_lib_file_and_save(name);
    remove_module_from_library_dir(name);

    log_styled_message_to_console("Module destroyed", ConsoleColors::GREEN);
}

/// Removes the module from the library's `lib.rs` file and writes it back to
/// the library.
///
/// # Arguments
///
/// `name` - The name of the module to remove.
fn remove_module_from_lib_file_and_save(name: &str) {
    let current_dir = std::env::current_dir().expect("Unable to get current directory");

    // We need the snake case and pascal case versions of the module name to
    // find the module's references in the lib.rs file.
    let module_name_snake_case = name.to_case(Case::Snake);
    let module_name_pascal_case = name.to_case(Case::Pascal);

    // The contents of the `src/lib.rs` file.
    let lib_contents = std::fs::read_to_string(current_dir.join("src").join("lib.rs"))
        .expect("Unable to read src/lib.rs file");

    // Create the mod and handle strings that we want to search for and remove
    // from the lib file.
    let mod_replace = format!("mod {};", &module_name_snake_case);
    let handle_replace = format!(
        "handle.add_class::<{}::{}>();",
        &module_name_snake_case, &module_name_pascal_case
    );
    let handle_replace_plugin = format!(
        "handle.add_tool_class::<{}::{}>();",
        &module_name_snake_case, &module_name_pascal_case
    );

    // Remove the `mod` declaration for the module.
    let file_contents_no_mod = lib_contents
        .lines()
        .filter(|&line| line.trim() != mod_replace)
        .collect::<Vec<_>>()
        .join("\n");

    // Remove the turbofish statements for the module.
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

    // Write the new contents to the lib file without any references to the
    // module that was removed.
    write_and_fmt("src/lib.rs", file_contents_no_mod_no_handles).expect("Unable to write lib file");
}

/// Removes the module from the library file system.
///
/// # Arguments
///
/// `name` - The name of the module to remove.
fn remove_module_from_library_dir(name: &str) {
    // We need the snake case version of the module name to get the mod file
    // name of the module.
    let module_name_snake_case = name.to_case(Case::Snake);

    // Get the path to the module's mod.rs file and remove it.
    let module_file_name = format!("src/{}.rs", &module_name_snake_case);
    let module_path = Path::new(&module_file_name);

    remove_file(module_path).expect("Unable to remove module file");
}

/// Removes the module's .gdns file from the Godot project.
///
/// # Arguments
///
/// `name` - The name of the module to remove.
/// `godot_project_name` - The name of the Godot project from the config.
fn remove_module_gdns_from_godot(name: &str, godot_project_name: String) {
    // We need the snake case version of the module name to get the gdns file
    // name of the module.
    let module_name_snake_case = name.to_case(Case::Snake);
    let gdns_file_name = format!("{}.gdns", &module_name_snake_case);

    // Get the parent directory of the library since it's also the directory
    // that contains the Godot project.
    let current_dir = std::env::current_dir().expect("Unable to get current directory");
    let parent_dir = current_dir
        .parent()
        .expect("Unable to get parent directory");

    // Use the parent directory, the Godot project directory, and the
    // `rust_modules` directory to get the path to the module's gdns file and
    // remove it.
    let gdns_path = parent_dir
        .join(&godot_project_name)
        .join("rust_modules")
        .join(gdns_file_name);
    remove_file(gdns_path).expect("Unable to remove module's gdns file");
}

/// Removes the plugin directory that corresponds to the module.
///
/// # Arguments
///
/// `name` - The name of the module to remove.
fn remove_godot_plugin_dir_if_exists(name: &str) {
    // We need the snake case version of the module name to get the name of
    // the plugin dir of the module.
    let module_name_snake_case = name.to_case(Case::Snake);

    // Get the parent directory since it contains the Godot project directory
    // which has the plugin directory for the module.
    let current_dir = std::env::current_dir().expect("Unable to get current directory");
    let parent_dir = current_dir
        .parent()
        .expect("Unable to get parent directory");

    // Create the path to the plugin directory.
    let godot_project_name = get_config_as_object().godot_project_name;
    let plugin_path = parent_dir
        .join(&godot_project_name)
        .join("addons")
        .join(&module_name_snake_case);

    if plugin_path.exists() {
        // Remove the plugin if it actually exists in the Godot project.
        std::fs::remove_dir_all(plugin_path)
            .expect("Unable to remove plugin directory from Godot project")
    }
}
