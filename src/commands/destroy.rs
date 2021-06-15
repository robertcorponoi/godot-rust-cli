use std::env::current_dir;
use std::fs::{read_to_string, remove_file};
use std::path::{Path, PathBuf};

use convert_case::{Case, Casing};
use walkdir::WalkDir;

use crate::config_utils::{get_config_as_object, remove_module_from_config_if_exists, Config};
use crate::file_utils::write_and_fmt;
use crate::log_utils::{log_styled_message_to_console, ConsoleColors};
use crate::path_utils::exit_if_not_lib_dir;

/// Removes a module by deleting its module file from the library and searching
/// the Godot project for the corresponding gdns file to remove.
///
/// # Arguments
///
/// `name` - The name of the module to remove.
pub fn destroy_module(name: &str) {
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
    let path_to_godot_project = parent_dir.join(&config.godot_project_name);

    remove_module_from_config_if_exists(&module_name_pascal_case, &mut config);

    remove_module_gdns_from_godot(&module_name_snake_case, &path_to_godot_project, &config);
    remove_module_from_lib_file(
        &module_name_snake_case,
        &module_name_pascal_case,
        &current_dir_path,
    );
    remove_module_from_library_dir(&module_name_snake_case);

    log_styled_message_to_console("Module destroyed", ConsoleColors::GREEN);
}

/// Removes all traces of a module from the lib.rs file.
///
/// # Arguments
///
/// `module_name_snake_case` - The snake case version of the module name.
/// `module_name_pascal_case` - The pascal case version of the module name.
/// `current_dir_path` - The path to the current directory.
fn remove_module_from_lib_file(
    module_name_snake_case: &str,
    module_name_pascal_case: &str,
    current_dir_path: &PathBuf,
) {
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
}

/// Removes the module's file from the library.
///
/// # Arguments
///
/// `module_name_snake_case` - The snake case version of the module name.
fn remove_module_from_library_dir(module_name_snake_case: &str) {
    let module_file_name = format!("src/{}.rs", &module_name_snake_case);
    let module_file_path = Path::new(&module_file_name);

    remove_file(module_file_path)
        .expect("Unable to remove the module file from the library while destroying the module");
}

/// Removes the module's gdns file from Godot project.
///
/// # Arguments
///
/// `module_name_snake_case` - The snake case version of the module name.
/// `godot_project_absolute_path` - The absolute path to the Godot project.
/// `config` - The current configuration.
fn remove_module_gdns_from_godot(
    module_name_snake_case: &str,
    godot_project_absolute_path: &PathBuf,
    config: &Config,
) {
    let library_name_snake_case = &config.name.to_case(Case::Snake);
    let gdns_file_name = format!("{}.gdns", &module_name_snake_case);

    // The first place we should check for the module to remove is either the
    // `rust_modules` folder in the plugin directory if it's a plugin or just
    // the `rust_modules` folder in the root directory of the Godot project
    // otherwise.
    let possible_gdns_path = if config.is_plugin {
        godot_project_absolute_path
            .join("addons")
            .join(&library_name_snake_case)
            .join("rust_modules")
            .join(&gdns_file_name)
    } else {
        godot_project_absolute_path
            .join("rust_modules")
            .join(&gdns_file_name)
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
            godot_project_absolute_path
                .join("addons")
                .join(&library_name_snake_case)
        } else {
            godot_project_absolute_path.to_owned()
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
}
