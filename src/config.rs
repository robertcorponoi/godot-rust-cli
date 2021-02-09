use serde::{Deserialize, Serialize};

/// The structure of the configuration file used by Godot Rust CLI to manage
/// the modules and properties of the library.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
  /// The name of the directory of the Godot project.
  pub godot_project_name: String,
  /// Tracks the modules created and destroyed through the cli.
  pub modules: Vec<String>,
}
