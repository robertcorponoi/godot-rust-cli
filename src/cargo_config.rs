use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::{env::current_dir, fs::create_dir_all, process::exit};

use crate::log_utils::log_error_to_console;

/// The structure of the config.toml file for the Rust library.
#[derive(Debug, Serialize, Deserialize)]
pub struct CargoConfig {
    pub env: ConfigEnv,
}

/// The structure of the `[env]` section in the configuration file.
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigEnv {
    /// The path to the Godot project.
    #[serde(rename = "GODOT_RUST_CLI_PROJECT_PATH")]
    pub godot_project_path: String,
}

impl CargoConfig {
    /// Creates a new instance of the config with the provided properties.
    ///
    /// # Arguments
    ///
    /// `godot_project_path`    - The path to the Godot project.
    pub fn new(godot_project_path: &str) -> CargoConfig {
        CargoConfig {
            env: ConfigEnv {
                godot_project_path: godot_project_path.to_string(),
            },
        }
    }

    /// Returns the Config as a pretty printed toml string.
    ///
    /// # Arguments
    ///
    /// `self` - The current Config object.
    pub fn to_string(&mut self) -> String {
        toml::to_string_pretty(self).expect("Unable to convert config to string")
    }

    /// Reads the contents of the `.cargo/config.toml` file and returns it.
    pub fn read() -> CargoConfig {
        // The path to the `.cargo/config.toml` file.
        let current_dir = current_dir().expect("Unable to run build, please try again");
        let config_file_path = current_dir.join(".cargo").join("config.toml");

        // Read the contents of the config file to a string.
        let rust_library_config_toml_string = read_to_string(config_file_path)
            .expect("Unable to read the Rust library's config.toml file");

        // Next, we deserialize it into our structure.
        let rust_library_config_toml: CargoConfig =
            toml::from_str(&rust_library_config_toml_string)
                .expect("Unable to parse the Rust library's config.toml file");

        rust_library_config_toml
    }

    /// Writes the provided Config to the `.cargo/config.toml` file.
    pub fn write(&mut self) {
        // The path to the `.cargo` directory.
        let current_dir = current_dir().expect("Unable to run build, please try again");
        let dotcargo_dir = current_dir.join(".cargo");

        // If this is the first time that the file is being written it might
        // not exist so we have to make sure to create the `.cargo` directory.
        match create_dir_all(&dotcargo_dir) {
            Ok(_) => (),
            Err(e) => {
                log_error_to_console(&e.to_string());
                exit(1);
            }
        }

        std::fs::write(dotcargo_dir.join("config.toml"), self.to_string())
            .expect("Unable to update contents of the Rust library's config.toml file");
    }
}
