use std::env::current_dir;
use std::fs::{read_to_string, remove_file};
use std::path::{Path, PathBuf};

use convert_case::{Case, Casing};
use walkdir::WalkDir;

use crate::config_utils::{get_config_as_object, remove_module_from_config_if_exists, Config};
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

    // Normalize the name of the module to remove.
    let module_name_snake_case = name.to_case(Case::Snake);
    let module_name_pascal_case = name.to_case(Case::Pascal);

    // Get the parent directory of the library since it's also the directory
    // that contains the Godot project.
    let current_dir = current_dir().expect("Unable to get current directory");
    let parent_dir = current_dir
        .parent()
        .expect("Unable to get parent directory");
    let godot_project_dir = parent_dir.join(&config.godot_project_name);

    // Remove the module from the config if it exists. If it doesn't exist,
    // then an error is thrown and we return early since there is nothing to
    // remove.
    remove_module_from_config_if_exists(&module_name_pascal_case, &mut config);

    // Remove the parts of the module from the Godot project. This includes the
    // gdns file for the module and the plugin directory if the module was a
    // plugin.
    remove_module_gdns_from_godot(&module_name_snake_case, &godot_project_dir, &config);

    // Remove the parts of the module from the library directory. This includes
    // the module's file and it's references from the `lib.rs` file.
    remove_module_from_lib_file_and_save(
        &module_name_snake_case,
        &module_name_pascal_case,
        &current_dir,
    );
    remove_module_from_library_dir(&module_name_snake_case);

    log_styled_message_to_console("Module destroyed", ConsoleColors::GREEN);
}

/// Removes the module from the library's `lib.rs` file and writes it back to
/// the library.
///
/// # Arguments
///
/// `module_name_snake_case` - The snake case version of the module to destroy.
/// `module_name_pascal_case` - The pascal case version of the module to destroy.
/// `current_dir` - The current directory.
fn remove_module_from_lib_file_and_save(
    module_name_snake_case: &str,
    module_name_pascal_case: &str,
    current_dir: &PathBuf,
) {
    // The contents of the `src/lib.rs` file.
    let lib_contents = read_to_string(current_dir.join("src").join("lib.rs"))
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
/// `module_name_snake_case` - The snake case version of the module to remove.
fn remove_module_from_library_dir(module_name_snake_case: &str) {
    // Get the path to the module's mod.rs file and remove it.
    let module_file_name = format!("src/{}.rs", &module_name_snake_case);
    let module_path = Path::new(&module_file_name);

    remove_file(module_path).expect("Unable to remove module file");
}

/// Removes the module's .gdns file from the Godot project.
///
/// # Arguments
///
/// `module_name_snake_case` - The snake case version of the module to remove.
/// `godot_project_dir` - The path to the Godot project.
/// `config` - The library's config.
fn remove_module_gdns_from_godot(
    module_name_snake_case: &str,
    godot_project_dir: &PathBuf,
    config: &Config,
) {
    let library_name_snake_case = &config.name.to_case(Case::Snake);
    let gdns_file_name = format!("{}.gdns", &module_name_snake_case);

    // The first place we should check for the module to remove is either the
    // `rust_modules` folder in the plugin directory if it's a plugin or just
    // the `rust_modules` folder in the root directory of the Godot project
    // otherwise.
    let possible_gdns_path = if config.is_plugin {
        godot_project_dir
            .join("addons")
            .join(&library_name_snake_case)
            .join("rust_modules")
            .join(&gdns_file_name)
    } else {
        godot_project_dir.join("rust_modules").join(&gdns_file_name)
    };

    if possible_gdns_path.exists() {
        // If this path exists, then we can just remove the module.
        remove_file(possible_gdns_path).expect("Unable to remove module's gdns file");
    } else {
        // Otherwise, we want to search a directory for the module. If the
        // module is a plugin, we can limit our search to the plugin directory.
        // Otherwise, we search the entire project since the user might have
        // moved it around.
        let search_dir = if config.is_plugin {
            godot_project_dir
                .join("addons")
                .join(&library_name_snake_case)
        } else {
            godot_project_dir.to_owned()
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
