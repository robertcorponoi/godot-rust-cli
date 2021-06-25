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
#[path = "./commands/target.rs"]
mod command_target;

#[path = "./utils/config.rs"]
mod config_utils;
#[path = "./utils/file.rs"]
mod file_utils;
#[path = "./utils/gdnlib.rs"]
mod gdnlib_utils;
#[path = "./utils/lib.rs"]
mod lib_utils;
#[path = "./utils/log.rs"]
mod log_utils;
#[path = "./utils/path.rs"]
mod path_utils;
#[path = "./utils/time.rs"]
mod time_utils;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Provides an easy way to incorporate Rust components into your Godot project")]
enum GodotRustCli {
    /// Creates a library for managing the Rust modules and creates the
    /// required gdnative files and directory structure within the Godot
    /// project.
    ///
    /// The name argument is the name of the project. This is used as the name
    /// of the library when creating it with cargo new.
    ///
    /// The godot-project-dir argument is the name of directory of the Godot
    /// project.
    ///
    /// The --plugin flag can be used to create a plugin library instead of a
    /// regular library. Plugin libraries will create the plugin structure
    /// within the Godot project and all Rust modules will be a part of that
    /// plugin.
    ///
    /// The --skip-build flag can be used to skip the initial build. This is
    /// mostly used for tests as skipping the build can cause Godot to throw
    /// errors about missing dynamic libraries for the project.
    ///
    /// # Examples
    ///
    /// ```
    /// // Creating a new library named platformer-modules for a Godot project
    /// // in the same directory named platformer.
    /// godot-rust-cli new platformer-modules ./platformer
    /// ```
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

        /// Indicates whether the library is for a plugin or not.
        #[structopt(long, short)]
        plugin: bool,

        /// Indicates whether automatic build of the library after creation
        /// should be skipped or not. The build is not necessary but ensures
        /// that there's no missing dynamic library error in Godot.
        #[structopt(long, short)]
        skip_build: bool,
    },

    /// Creates a new rust module within the library's file system, adds its
    /// entry to the lib.rs file, and creates a gdns file for it within the
    /// Godot project.
    ///
    /// The name of the module should be PascalCase.
    ///
    /// # Examples
    ///
    /// ```
    /// // Creates a new module named Player
    /// godot-rust-cli create Player
    /// ```
    Create {
        /// The name of the module to create. The component name should be
        /// PascalCase with examples including 'Player', 'Princess', 'Mob',
        /// etc.
        #[structopt()]
        name: String,
    },

    /// Deletes a Rust module from the library's file system, removes its entry
    /// from the lib.rs file, and deletes it from the Godot project structure.
    ///
    /// The name passed to this command should be the same name that was used
    /// when the module was created.
    ///
    /// # Examples
    ///
    /// ```
    /// // Creates an destroys a module named Player.
    /// godot-rust-cli create Player
    /// godot-rust-cli destroy Player
    /// ```
    Destroy {
        /// The name of the module to destroy. This should be the same name
        /// that was used when the module was created.
        #[structopt()]
        name: String,
    },

    /// Builds the dynamic library/libraries for the project and copies them to
    /// the Godot project.
    ///
    /// The --watch flag can be passed to have godot-rust-cli watch the src
    /// directory for changes and rebuild automatically.
    ///
    /// The --release flag can be passed to have godot-rust-cli create a
    /// release build for the library instead of a debug build.
    ///
    /// The --build-all-targets flag can be passed to have godot-rust-cli run a
    /// build for every target in the config file. By default, a build will
    /// only be created for the native platform.
    ///
    /// # Examples
    ///
    /// ```
    /// // Running the default build for the native target.
    /// godot-rust-cli build
    /// ```
    ///
    /// ```
    /// // Running the default build and watching the src directory for
    /// // changes.
    /// godot-rust-cli build --watch
    /// ```
    ///
    /// ```
    /// // Creating a release build instead of the default debug build.
    /// godot-rust-cli build --release
    /// ```
    ///
    /// ```
    /// // Building for all of the targets in the config file.
    /// godot-rust-cli build --build-all-targets
    /// ```
    Build {
        /// Indicates whether components should be watched for changes and be
        /// rebuild automatically or not.
        #[structopt(long, short)]
        watch: bool,

        /// Indicates whether the build is a release build or not.
        #[structopt(long, short)]
        release: bool,

        /// Indicates whether all of the targets defined in the config should
        /// be built or not. By default, if this is not provided, then just
        /// the target for the native platform will be build.
        #[structopt(long, short)]
        build_all_targets: bool,
    },

    /// Adds a new target to the list of targets that the library can be built
    /// for. A target only needs to be added if you are trying to target
    /// something other than your native target.
    ///
    /// Targets are a more advanced feature so make sure to check the
    /// documentation on them specifically.
    ///
    /// # Examples
    ///
    /// ```
    /// // Adding a 64 bit Linux target.
    /// godot-rust-cli add-target "x86_64-unknown-linux-gnu - Linux.64"
    /// ````
    AddTarget {
        /// The name of the target to add. Check the documentation on targets
        /// to see the list of available targets that can be used. If the
        /// target provided conflicts with the platform of another target, the
        /// --overwrite flag must be used to overwrite the existing target.
        #[structopt()]
        name: String,

        /// Indicates whether an identical target or a target for the same
        /// platform as the specified target should be overwritten with the
        /// specified target or not.
        #[structopt(long, short)]
        overwrite: bool,
    },

    /// Removes a target from the list of targets that the library can be built
    /// for.
    ///
    /// # Examples
    ///
    /// ```
    /// // Removing a 64 bit Linux target.
    /// godot-rust-cli remove-target "x86_64-unknown-linux-gnu - Linux.64"
    /// ```
    RemoveTarget {
        /// The name of the target to remove. This should be the same name that
        /// was used when the target was added.
        #[structopt()]
        name: String,
    },
}

fn main() {
    match GodotRustCli::from_args() {
        GodotRustCli::New {
            name,
            godot_project_dir,
            plugin,
            skip_build,
        } => command_new::create_library(&name, godot_project_dir, plugin, skip_build),
        GodotRustCli::Create { name } => command_create::create_module(&name),
        GodotRustCli::Destroy { name } => command_destroy::destroy_module(&name),
        GodotRustCli::Build {
            watch,
            release,
            build_all_targets,
        } => {
            if watch {
                command_build::build_library_and_watch_for_changes(release, build_all_targets);
            } else {
                command_build::build_library(release, build_all_targets);
            }
        }
        GodotRustCli::AddTarget { name, overwrite } => command_target::add_target(&name, overwrite),
        GodotRustCli::RemoveTarget { name } => command_target::remove_target(&name),
    }
}
