#[macro_use]

mod definitions;

#[path = "./commands/build.rs"]
mod command_build;
#[path = "./commands/create.rs"]
mod command_create;
#[path = "./commands/destroy.rs"]
mod command_destroy;
#[path = "./commands/new.rs"]
mod command_new;

#[path = "./utils/config.rs"]
mod config_utils;
#[path = "./utils/dynamic_library.rs"]
mod dynamic_library_utils;
#[path = "./utils/file.rs"]
mod file_utils;
#[path = "./utils/log.rs"]
mod log_utils;
#[path = "./utils/path.rs"]
mod path_utils;
#[path = "./utils/time.rs"]
mod time_utils;
#[path = "./utils/lib.rs"]
mod lib_utils;

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

        /// Indicates whether automatic build of the library after creation
        /// should be skipped or not. The build is not necessary but ensures
        /// that there's no missing dynamic library error in Godot.
        #[structopt(long, short)]
        skip_build: bool,
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
            skip_build,
        } => command_new::create_library(&name, godot_project_dir, skip_build),
        GodotRustCli::Create { name } => command_create::create_module(&name, false),
        GodotRustCli::Destroy { name } => command_destroy::destroy_module(&name),
        GodotRustCli::Build { watch } => {
            if watch {
                command_build::build_library_and_watch();
            } else {
                command_build::build_library();
            }
        }
        GodotRustCli::Plugin { name } => command_create::create_module(&name, true),
    }
}
