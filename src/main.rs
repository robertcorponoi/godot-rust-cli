#[macro_use]

mod utils;
mod commands;
mod config;
mod definitions;

use std::path::PathBuf;
use structopt::StructOpt;

/// Classifies the commands that can be used by the user.
#[derive(Debug, StructOpt)]
#[structopt(about = "Provides an easy way to incorporate Rust components into your Godot project")]
enum GodotRustCli {
  /// Creates a library to hold one or more Rust components.
  ///
  /// Usage: godot-rust-cli new platformer-modules ./platformer
  New {
    /// The name of the library that will contain the Rust components. The
    /// library created is itself a cargo package so it needs to adhere to
    /// cargo naming standards.
    #[structopt()]
    name: String,

    /// The relative path to the directory of the Godot project that this
    /// library of components is for.
    #[structopt(parse(from_os_str))]
    godot_project_dir: PathBuf,
  },

  /// Creates a new Rust component inside the current library.
  ///
  /// Usage: godot-rust-cli create Player
  Create {
    /// The name of the component to create. The component name should be
    /// PascalCase with examples including 'Player', 'Princess', 'Mob', etc.
    #[structopt()]
    name: String,
  },

  /// Destroys an existing component.
  ///
  /// Usage: godot-rust-cli destroy Player
  Destroy {
    /// The name of the component to destroy. This should be the same name that
    /// was used when the component was created.
    #[structopt()]
    name: String,
  },

  /// Builds the dynamic libraries for the Rust components and copies them to
  /// the Godot project so that they can be used.
  ///
  /// Usage: godot-rust-cli build
  Build {
    /// Indicates whether components should be watched for changes and be rebuilt
    /// automatically or not.
    #[structopt(long, short)]
    watch: bool,
  },

  /// Creates a plugin in the library and Godot project.
  ///
  /// Usage: godot-rust-cli plugin "Toml Parser"
  Plugin {
    /// The name of the plugin.
    #[structopt()]
    name: String,
  },
}

fn main() {
  match GodotRustCli::from_args() {
    GodotRustCli::New {
      name,
      godot_project_dir,
    } => commands::command_new(&name, godot_project_dir),
    GodotRustCli::Create { name } => commands::command_create(&name, false),
    GodotRustCli::Destroy { name } => commands::command_destroy(&name),
    GodotRustCli::Build { watch } => {
      if watch {
        commands::command_build_watch();
      } else {
        commands::command_build();
      }
    }
    GodotRustCli::Plugin { name } => commands::command_create(&name, true),
  }
}
