use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::fs::write;
use std::path::PathBuf;
use std::process::exit;

use crate::log_utils::{log_styled_message_to_console, ConsoleColors};
use crate::path_utils::get_config_path;

/// Returns the contents of the configuration file as an object so that the
/// key-value pairs can be operated on.
pub fn get_config_as_object() -> Config {
    let project_toml_string =
        read_to_string(get_config_path()).expect("Unable to read godot-rust-cli.toml");

    return toml::from_str(&project_toml_string).expect("Unable to parse godot-rust-cli.toml");
}

/// Creates the initial config file and writes it to the library.
///
/// `godot_project_absolute_path` - The absolute path to the Godot project.
pub fn create_initial_config(godot_project_absolute_path: &PathBuf) {
    let config = Config {
        godot_project_name: godot_project_absolute_path
            .file_name()
            .unwrap()
            .to_str()
            .expect("Unable to convert Godot file name to str")
            .to_string(),
        modules: vec![],
    };
    let config_string = toml::to_string(&config).expect("Unable to convert config to string");
    write("godot-rust-cli.toml", config_string).expect("Unable to create godot-rust-cli.toml file");
}

/// Writes the new contents to the config file.
///
/// # Arguments
///
/// `new_config_contents` - The new contents of the config file.
pub fn set_config(new_config_contents: &mut Config) {
    let project_toml_path = get_config_path();
    let new_project_toml_string = toml::to_string(&new_config_contents)
        .expect("Unable to convert godot-rust-cli.toml to string");

    match write(project_toml_path, new_project_toml_string) {
        Ok(_) => (),
        Err(e) => {
            log_styled_message_to_console(&e.to_string(), ConsoleColors::RED);
            exit(1);
        }
    }
}

/// Adds a module to the config file.
///
/// # Arguments
///
/// `module_name` - The name of the module to add to the config file.
pub fn add_module_to_config(module_name: &str) {
    let mut config = get_config_as_object();
    let module_name_pascal_case = &module_name.to_case(Case::Pascal);

    config.modules.push(module_name_pascal_case.to_string());
    set_config(&mut config);
}

/// Removes a module from the config file if it exists.
///
/// # Arguments
///
/// `module_name` - The name of the module to remove from the config file.
pub fn remove_module_from_config_if_exists(module_name: &str) {
    let mut config = get_config_as_object();
    let module_name_pascal_case = module_name.to_case(Case::Pascal);
    let module_exists_in_config = is_module_in_config(&config.modules, &module_name_pascal_case);

    if !module_exists_in_config {
        log_styled_message_to_console("The module to remove doesn't exist", ConsoleColors::RED);
        exit(1);
    }

    let index_of_module_to_remove = config
        .modules
        .iter()
        .position(|x| *&x == &module_name_pascal_case)
        .expect("Unable get index of module to remove");
    config.modules.remove(index_of_module_to_remove);

    set_config(&mut config);
}

/// Indicates whether a module is present in the config or not.
///
/// # Arguments
///
/// `modules` - The modules from the config file.
/// `module_name` - The module to check if exists or not.
pub fn is_module_in_config(modules: &Vec<String>, module_name: &str) -> bool {
    if modules.iter().any(|i| i == module_name) {
        true
    } else {
        false
    }
}

/// The stucture of the configuration file.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// The name of the directory of the Godot project.
    pub godot_project_name: String,
    /// Tracks the modules created and destroyed through the cli.
    pub modules: Vec<String>,
}
