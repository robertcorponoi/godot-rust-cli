use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::read_to_string;
use std::fs::write;
use std::path::{Path, PathBuf};
use std::process::exit;

use crate::config_utils::Config;
use crate::log_utils::log_error_to_console;
use convert_case::{Case, Casing};

/// The structure of the gdnlib file.
#[derive(Debug, Serialize, Deserialize)]
pub struct Gdnlib {
    general: GdnlibGeneral,
    entry: HashMap<String, String>,
    dependencies: HashMap<String, String>,
}

/// The structure of the general section of the gdnlib file.
#[derive(Debug, Serialize, Deserialize)]
pub struct GdnlibGeneral {
    pub singleton: bool,
    pub load_once: bool,
    pub symbol_prefix: String,
    pub reloadable: bool,
}

/// Returns the path to the gdnlib file from the root of the library.
///
/// `config` - The configuration object.
pub fn get_path_to_gdnlib_file(config: &Config) -> PathBuf {
    let current_dir = current_dir()
        .expect("Unable to get current directory while getting the path to the gdnlib file.");
    let parent_dir = current_dir
        .parent()
        .expect("Unable to get parent directory while getting the path to the gndlib file.");

    let library_name_snake_case = &config.name.to_case(Case::Snake);
    if config.is_plugin {
        return parent_dir
            .join(&config.godot_project_name)
            .join("addons")
            .join(library_name_snake_case)
            .join("gdnative")
            .join(format!("{}.gdnlib", library_name_snake_case));
    } else {
        return parent_dir
            .join(&config.godot_project_name)
            .join("gdnative")
            .join(format!("{}.gdnlib", library_name_snake_case));
    };
}

/// Creates the initial gdnlib and saves it to the Godot project.
///
/// # Arguments
///
/// `config` - The configuration object.
pub fn create_initial_gdnlib(config: &Config) -> Gdnlib {
    let gdnlib_general = GdnlibGeneral {
        singleton: true,
        load_once: true,
        symbol_prefix: "godot_".to_string(),
        reloadable: true,
    };
    let mut gdnlib = Gdnlib {
        general: gdnlib_general,
        entry: HashMap::new(),
        dependencies: HashMap::new(),
    };

    save_gdnlib_to_file(config, &mut gdnlib);

    return gdnlib;
}

/// Returns the config as an object that can be operated on.
///
/// `config` - The configuration object.
pub fn get_gdnlib_as_object(config: &Config) -> Gdnlib {
    let gdnlib_file_path = get_path_to_gdnlib_file(config);
    let gdnlib_as_string =
        read_to_string(gdnlib_file_path).expect("Unable to read gdnlib file to string.");

    return toml::from_str(&gdnlib_as_string).expect("Unable to parse gdnlib file.");
}

/// Saves the gdnlib file to the Godot project directory.
///
/// # Arguments
///
/// `config` - The configuration object.
/// `gdnlib_file` - The gdnlib object to save.
pub fn save_gdnlib_to_file(config: &Config, gdnlib: &mut Gdnlib) {
    let gdnlib_file_path = get_path_to_gdnlib_file(config);
    let gdnlib_as_string =
        toml::to_string_pretty(&gdnlib).expect("Unable to convert gdnlib to string.");
    match write(gdnlib_file_path, gdnlib_as_string) {
        Ok(_) => (),
        Err(e) => {
            log_error_to_console(&e.to_string());
            exit(1);
        }
    }
}

/// Adds a target to the gdnlib and saves it.
///
/// # Arguments
///
/// `target` - The target to add.
/// `platform` - The platform that the target is for.
/// `gdnlib` - The gdnlib object to add the target to.
pub fn add_target_to_gdnlib(target: &str, platform: &str, gdnlib: &mut Gdnlib) {}
