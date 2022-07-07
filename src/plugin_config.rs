use std::{fs::read_to_string, path::PathBuf};

use serde::{Deserialize, Serialize};

/// The structure of a Godot project's plugin.cfg if the project is a plugin.
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginConfig {
    /// A reference to the plugin struct.
    pub plugin: PluginConfigPlugin,
}

/// The fields of the plugin.cfg that are under the [plugin] tag.
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginConfigPlugin {
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: String,
    pub script: String,
}

impl PluginConfig {
    /// Creates a new instance of the PluginConfig with the provided
    /// properties.
    ///
    /// # Arguments
    ///
    /// `name` - The name of the plugin.
    /// `script_path` - The path to the script for the plugin.
    pub fn new(name: &str, script_path: &str) -> PluginConfig {
        PluginConfig {
            plugin: PluginConfigPlugin {
                name: name.to_string(),
                description: "".to_string(),
                author: "".to_string(),
                version: "1.0".to_string(),
                script: script_path.to_string(),
            },
        }
    }

    /// Returns the PluginConfig as a pretty printed toml string.
    pub fn to_string(&mut self) -> String {
        toml::to_string_pretty(self).expect("Unable to convert plugin config to string")
    }

    /// Reads and returns the parsed contents of the `plugin.cfg` file.
    ///
    /// # Arguments
    ///
    /// `path` - The path to the `plugin.cfg` file.
    #[allow(dead_code)]
    pub fn read(path: PathBuf) -> PluginConfig {
        // Read the contents of the plugin config file to a string.
        let plugin_config_toml_string =
            read_to_string(path).expect("Unable to read the Godot project's plugin.cfg file");

        // Next, we deserialize it into our structure.
        let plugin_config_toml: PluginConfig = toml::from_str(&plugin_config_toml_string)
            .expect("Unable to parse the Godot project's plugin.cfg file");

        plugin_config_toml
    }

    /// Writes the provided PluginConfig to the `plugin.cfg` file.
    ///
    /// # Arguments
    ///
    /// `path` - The path to write the `plugin.cfg` file to.
    pub fn write(&mut self, path: PathBuf) {
        std::fs::write(path, self.to_string())
            .expect("Unable to update contents of the Godot project's plugin.cfg file");
    }
}
