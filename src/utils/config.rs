use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::fs::read_to_string;
use std::fs::write;
use std::path::{Path, PathBuf};
use std::process::exit;

use crate::log_utils::{log_styled_message_to_console, ConsoleColors};

/// The stucture of the configuration file.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// The name of the library.
    pub name: String,
    /// The name of the directory of the Godot project.
    pub godot_project_name: String,
    /// Indicates whether the library is for a plugin or not.
    pub is_plugin: bool,
    /// Tracks the modules created and destroyed through the cli.
    pub modules: Vec<String>,
}

/// Returns the path to the configuration file.
pub fn get_path_to_config_file() -> PathBuf {
    let curr_dir = current_dir().expect("Unable to get current directory");
    return Path::new(&curr_dir).join("godot-rust-cli.json");
}

/// Creates the initial configuration and saves it to a json file.
///
/// # Arguments
///
/// `library_name` - The name of the library.
/// `godot_project_name` - The name of the Godot project.
/// `is_library` - Indicates whether the library is for a plugin or not.
pub fn create_initial_config(library_name: String, godot_project_name: String, is_plugin: bool) -> Config {
    let config = Config {
        name: library_name,
        godot_project_name: godot_project_name,
        is_plugin: is_plugin,
        modules: vec![],
    };
    let config_as_json =
        serde_json::to_string_pretty(&config).expect("Unable to create initial configuration");

    write("godot-rust-cli.json", config_as_json).expect("Unable to create configuration file");

    return config;
}

/// Returns the configuration as an object that can be operated on.
pub fn get_config_as_object() -> Config {
    let config_file_path = get_path_to_config_file();
    let config_as_string =
        read_to_string(config_file_path).expect("Unable to read configuration file");

    return serde_json::from_str(&config_as_string).expect("Unable to parse configuration file");
}

/// Saves the configuration file to the library directory.
///
/// # Arguments
///
/// `config` - The configuration to save.
pub fn save_config_to_file(config: &mut Config) {
    let config_file_path = get_path_to_config_file();
    let config_as_string =
        serde_json::to_string_pretty(&config).expect("Unable to parse configuration");

    match write(config_file_path, config_as_string) {
        Ok(_) => (),
        Err(e) => {
            log_styled_message_to_console(&e.to_string(), ConsoleColors::RED);
            exit(1);
        }
    }
}

/// Adds a module to the configuration file and saves it.
///
/// # Arguments
///
/// `module_name` - The name of the module to add to the configuration file.
/// `config` - Can be passed if the config is already in memory.
pub fn add_module_to_config(module_name: &str, config: &mut Config) {
    config.modules.push(module_name.to_string());
    save_config_to_file(config);
}

/// Indicates whether a module is present in the config or not.
///
/// # Arguments
///
/// `module_name` - The module to check if exists or not.
/// `config` - The configuration file.
pub fn is_module_in_config(module_name: &str, config: &mut Config) -> bool {
    return config.modules.iter().any(|i| i == module_name);
}

/// Removes a module from the config file if it exists.
///
/// # Arguments
///
/// `module_name` - The name of the module to remove from the config file.
/// `config` - The configuration file.
pub fn remove_module_from_config_if_exists(module_name: &str, config: &mut Config) {
    let module_exists_in_config = is_module_in_config(&module_name, config);

    if !module_exists_in_config {
        log_styled_message_to_console("The module to remove doesn't exist", ConsoleColors::RED);
        exit(1);
    }

    let index = config
        .modules
        .iter()
        .position(|x| *x == module_name)
        .unwrap();
    config.modules.remove(index);

    save_config_to_file(config);
}
