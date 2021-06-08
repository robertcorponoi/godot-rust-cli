use regex::Regex;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::read_to_string;

use convert_case::{Case, Casing};

use crate::config_utils::{get_config_as_object, Config};

/// Returns the contents of the lib.rs file as a string.
pub fn get_lib_file_contents() -> String {
    let current_dir = current_dir().expect("Unable to get current directory");
    let lib_file_contents =
        read_to_string(current_dir.join("src").join("lib.rs")).expect("Unable to read lib file");

    return lib_file_contents;
}

/// Takes a regex pattern to find matching lines and then another regex pattern
/// to find the module name within that match to find where the next module
/// definition can be placed.
///
/// # Arguments
///
/// `line_pattern` - The regex pattern to use to look for the line before the insert location.
/// `is_first` - Indicates whether this is the first module being added.
/// `config` - An optional parameter to pass the config to use.
/// `lib_contents` - An optional parameter to pass the lib contents to use.
pub fn get_insert_location(
    line_pattern: &str,
    is_first: bool,
    config: Option<Config>,
    lib_contents: Option<String>,
) -> (usize, HashMap<String, String>) {
    // Either use the config passed to us or get a local copy of it.
    let config_to_use = match config {
        Some(v) => v,
        None => get_config_as_object(),
    };
    // Either use the lib contents passed to us or get a local copy of it.
    let lib_file_contents_to_use = match lib_contents {
        Some(v) => v,
        None => get_lib_file_contents(),
    };

    // Since we want to add our new module after all of the currently existing
    // ones, we need to keep updating the insert position to the end position of
    // the last module found.
    let mut insert_pos = 0;

    // Create a copy of the modules hashmap so that we don't have to modify the
    // original one.
    let modules_copy = config_to_use.modules.clone();

    // Create the regex pattern used to check for the lines that might contain
    // our module definitions.
    let line_regex = Regex::new(line_pattern).expect("Unable to create regex");

    for line_match in line_regex.find_iter(&lib_file_contents_to_use) {
        if is_first {
            // If this is the first module to be added, meaning there's only 1
            // entry in the modules hashmap, then we don't have to search for
            // other modules and can just write it within the init function.
            return (line_match.end(), modules_copy);
        }
        insert_pos = line_match.end();
    }

    (insert_pos, modules_copy)
}

/// Adds a module to the lib.rs file.
///
/// `module_name` - The name of the module to add.
/// `is_plugin` - Indicates whether the module is a plugin or not.
/// `config` - Optionally, the config can be passed to be used.
pub fn add_module_to_lib(module_name: &str, is_plugin: bool, config: Option<Config>) {
    let config_to_use = match config {
        Some(v) => v,
        None => get_config_as_object(),
    };
    let lib_file_contents = get_lib_file_contents();

    // The position of where we should insert the `mod` statement for the
    // module.
    let module_mod_insert_location = get_insert_location(
        "mod.*;",
        false,
        Some(config_to_use),
        Some(lib_file_contents),
    );

    // Insert the new module's mod line after the last module's mod line.
    let mod_line = format!("mod {};", module_name);
    lib_file_contents.insert_str(module_mod_insert_location.0, &mod_line);

    // Next we do the same thing for the handle turbofish statement for the
    // module. However, this is different than the mod statement because if this
    // is the first module created, then we need to look for the init function.
    // If this is not the first module then we look for an existing module's
    // turbofish statement.

    let handle_insert_location_first = get_insert_location(
        "init.*\\{",
        true,
        Some(config_to_use),
        Some(lib_file_contents),
    );

    let handle_insert_location_normal = get_insert_location(
        "handle.*;",
        true,
        Some(config_to_use),
        Some(lib_file_contents),
    );

    // Insert the module or plugin turbofish at the start of the init function or
    // after the last module's turbofish.
    let module_name_pascal_case = &module_name.to_case(Case::Pascal);
    let handle_line = if is_plugin {
        format!(
            "handle.add_tool_class::<{}::{}>();",
            &module_name, module_name_pascal_case
        )
    } else {
        format!(
            "handle.add_class::<{}::{}>();",
            &module_name, module_name_pascal_case
        )
    };

    let handle_insert_location_to_use = if config_to_use.modules.len() == 0 {
        handle_insert_location_first
    } else {
        handle_insert_location_normal
    };

    lib_file_contents.insert_str(handle_insert_location_to_use.0, &handle_line);
}
