use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::fs::read_to_string;
use std::fs::write;
use std::path::{Path, PathBuf};
use std::process::exit;

use crate::log_utils::{log_error_to_console, log_info_to_console, log_success_to_console};
use convert_case::{Case, Casing};

/// The stucture of the configuration file.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// The name of the library.
    /// Added v0.3.0
    pub name: String,
    /// The version of godot-rust-cli being used for this library.
    /// Added v0.4.0
    pub cli_version: String,
    /// The name of the directory of the Godot project.
    /// Added v0.4.0
    pub godot_project_dir_name: String,
    /// Indicates whether the library is for a plugin or not.
    /// Added v0.3.0
    pub is_plugin: bool,
    /// The platforms to build the library for.
    /// Added v0.4.0
    pub platforms: Vec<String>,
    /// Tracks the modules created and destroyed through the cli.
    /// Added v0.1.0
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
/// `godot_project_dir_name` - The name of the Godot project.
/// `is_library` - Indicates whether the library is for a plugin or not.
pub fn create_initial_config(
    library_name: String,
    godot_project_dir_name: String,
    is_plugin: bool,
) -> Config {
    let cli_version = env!("CARGO_PKG_VERSION").to_string();
    let config = Config {
        name: library_name,
        cli_version,
        godot_project_dir_name: godot_project_dir_name,
        is_plugin: is_plugin,
        platforms: vec![],
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
            log_error_to_console(&e.to_string());
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
    // If the library is for a plugin, and the module is the root plugin module,
    // we don't add it to the config since it can't be removed.
    if config.is_plugin {
        let config_name_snake_case = &config.name.to_case(Case::Snake);
        let module_name_snake_case = &module_name.to_case(Case::Snake);

        if module_name_snake_case == config_name_snake_case {
            return;
        }
    }

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
        log_info_to_console("The module to remove doesn't exist.");
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

/// Adds a platform to the config file if it doesn't already exist.
///
/// # Arguments
///
/// `platform` - The platform to add to the configuration file.
/// `config` - Can be passed if the config is already in memory.
pub fn add_platform_to_config(platform: &str, config: &mut Config) {
    let platform_lowercase = platform.to_lowercase();

    if is_platform_in_config(&platform_lowercase, config) {
        log_info_to_console(&format!(
            "[add-platform] {} is already in the config.",
            &platform
        ));
        exit(1);
    }
    config.platforms.push(platform_lowercase);
    save_config_to_file(config);

    log_success_to_console(&format!(
        "[add-platform] Added {} to the config.",
        &platform
    ));
}

/// Indicates whether a platform was added to the config or not.
///
/// # Arguments
///
/// `platform` - The platform to check if exists or not.
/// `config` - The configuration file.
pub fn is_platform_in_config(platform: &str, config: &mut Config) -> bool {
    return config.platforms.iter().any(|i| i == platform);
}

/// Removes a platform from the config file if it exists.
///
/// # Arguments
///
/// `platform` - The platform to remove from the config file.
/// `config` - The configuration file.
pub fn remove_platform_from_config_if_exists(platform: &str, config: &mut Config) {
    let platform_lowercase = platform.to_lowercase();

    if !is_platform_in_config(&platform_lowercase, config) {
        log_info_to_console(&format!(
            "[remove-platform] {} can't be removed because doesn't exist in the config.",
            &platform
        ));
        exit(1);
    }

    let index = config
        .platforms
        .iter()
        .position(|x| *x == platform_lowercase)
        .unwrap();
    config.platforms.remove(index);

    save_config_to_file(config);

    log_success_to_console(&format!(
        "[remove-platform] {} removed from the config.",
        &platform
    ));
}
