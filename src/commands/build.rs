use chrono::offset::Local;
use convert_case::{Case, Casing};
use notify::{op, raw_watcher, RawEvent, RecursiveMode, Watcher};
use std::fs::create_dir_all;
use std::path::Path;
use std::process::{exit, Command};
use std::sync::mpsc::channel;

use crate::config_utils::{get_config_as_object, Config};
use crate::file_utils::copy_file_to_location;
use crate::log_utils::{
    log_error_to_console, log_info_to_console, log_success_to_console, log_version,
};
use crate::path_utils::get_dynamic_library_directory_path;
use crate::time_utils::get_current_datetime_formatted;

/// Runs the cargo build command to build the dynamic libraries and copy them
/// over to the bin folder in the Godot project.
///
/// The default build command will build the library using the native target.
/// For instance, if you're developing the library on 64-bit Windows then it
/// will build for 64-bit Windows. Targets can be added using the --add-target
/// command and these targets can built along with the native target by running
/// godot-rust-cli build --all.
///
/// Check out the documentation on adding targets for more information about
/// how to add more targets and how they work.
///
/// # Arguments
///
/// `is_release` - Indicates whether the build is a release build or not.
/// `build_all_targets` - Indicates whether all of the targets should be built instead of just the native target.
pub fn build_library(is_release: bool, build_all_targets: bool) {
    log_version();
    log_info_to_console("build starting...");

    let current_dir = std::env::current_dir()
        .expect("Unable to get current directory while building the library");
    let parent_dir = current_dir
        .parent()
        .expect("Unable to get parent directory while building the library");

    let config = get_config_as_object();
    let library_name_snake_case = &config.name.to_case(Case::Snake);

    if config.targets.len() == 0 {
        // If there's no targets left in the config, let the user know.
        log_info_to_console("There are no targets to build for in the config. If you accidently deleted your native target then run godot-rust-cli reset-targets.");
        exit(1);
    }

    // Build for the native target by default.
    let native_target: &str = env!("TARGET");
    build_for_target(
        parent_dir,
        &config,
        &library_name_snake_case,
        &native_target,
        is_release,
    );

    // If the user wants to build for all of the targets, we run the
    // `build_for_target` function for each target in the configuration.
    if build_all_targets {
        for (_platform, target) in &config.targets {
            build_for_target(
                parent_dir,
                &config,
                &library_name_snake_case,
                &target,
                is_release,
            );
        }
    }

    // Let the user know that the build is complete.
    log_success_to_console("build complete");
}

/// Watches the src directory in the library for changes and rebuilds the
/// library when changes are detected.
///
/// # Arguments
///
/// `is_release` - Indicates whether the build is a release build or not.
/// `build_all_targets` - Indicates whether all of the targets should be built instead of just the native target.
pub fn build_library_and_watch_for_changes(is_release: bool, build_all_targets: bool) {
    let (tx, rx) = channel();

    build_library_with_timestamp(is_release, build_all_targets);

    let mut last_checked = Local::now();
    let mut watcher =
        raw_watcher(tx).expect("Unable to create watcher to watch library for changes");
    let current_dir = std::env::current_dir()
        .expect("Unable to get current directory while attempting to watch library for changes");

    watcher
        .watch(current_dir.join("src"), RecursiveMode::Recursive)
        .expect("Unable to watch library directory for changes");
    loop {
        match rx.recv() {
            Ok(RawEvent {
                path: Some(_path),
                op: Ok(op),
                cookie: _,
            }) => {
                if op.contains(op::WRITE) {
                    let now = Local::now();
                    if (now - last_checked).num_seconds() == 0 {
                        build_library_with_timestamp(is_release, build_all_targets);
                    }
                    last_checked = Local::now();
                }
            }
            Ok(_event) => log_error_to_console("broken event"),
            Err(e) => log_error_to_console(&e.to_string()),
        }
    }
}

/// Runs the `build_library` function to build the library and copy the
/// dynamic library file to the Godot project.
///
/// In addition to that, it also logs the datetime that the build was
/// completed as `YYYY-MM-DD HH:MM::SS and lets the user know that it is
/// waiting for changes before building again.
///
/// # Arguments
///
/// `is_release` - Indicates whether the build is a release build or not.
/// `build_all_targets` - Indicates whether all of the targets should be built instead of just the native target.
pub fn build_library_with_timestamp(is_release: bool, build_all_targets: bool) {
    build_library(is_release, build_all_targets);

    // After the build we want to show a message letting the user know that the
    // build has finished and is waiting for futher changes before rebuilding.
    log_info_to_console("");
    log_info_to_console(&format!(
        "[{}] {}",
        get_current_datetime_formatted(),
        "waiting for changes..."
    ));
}

/// Builds the specified target using either the regular cargo build command if
/// the build is for the native target or `cross` for cross platform builds.
///
/// # Arguments
///
/// `parent_dir` - The path to the parent directory of the library.
/// `config` - A reference to the library's configuration.
/// `library_name_snake_case` - The snake case version of the library name.
/// `target` - The target to run the build for.
/// `is_release` - Indicates whether the build is a release build or not.
fn build_for_target(
    parent_dir: &Path,
    config: &Config,
    library_name_snake_case: &str,
    target: &str,
    is_release: bool,
) {
    let native_target: &str = env!("TARGET");

    // The path to the directory that contains the dynamic library file.
    let dynamic_library_directory = get_dynamic_library_directory_path(target, is_release);

    // The prefix of the dynamic library file name used to build the file name.
    let dynamic_library_file_name_prefix = std::env::consts::DLL_PREFIX;

    // The extension of the dynamic library file name used to build the file
    // name.
    let dynamic_library_file_name_extension = std::env::consts::DLL_SUFFIX;

    // Build the name of the dynamic library based on the directory, the prefix,
    // and then extension.
    let dynamic_library_file_name = format!(
        "{}{}{}",
        &dynamic_library_file_name_prefix,
        &library_name_snake_case,
        &dynamic_library_file_name_extension
    );
    let dynamic_library_file_path = dynamic_library_directory.join(&dynamic_library_file_name);

    // The base path to the Godot project's directory.
    let godot_project_dir_path = parent_dir.join(&config.godot_project_name);

    // Let the user know that the build has officially started.
    log_info_to_console(&format!("building for {}...", &target));

    if target == native_target {
        // If the target to build for is the native target, then we can just
        // run the regular cargo build.
        run_cargo_build_command(is_release);
    } else {
        // Otherwise we have to invoke the `cross` binary to build for the
        // cross-platform target.
        run_cross_build_command(&target, is_release);
    }

    // Build the path to the bin folder in the Godot project where the dynamic
    // library should be copied to.
    let godot_project_bin_path = if config.is_plugin {
        godot_project_dir_path
            .join("addons")
            .join(&library_name_snake_case)
            .join("bin")
    } else {
        parent_dir
            .join(&config.godot_project_name)
            .join("gdnative")
            .join("bin")
            .join(&target)
    };
    // Create the directory for the dynamic libraries in the Godot project if
    // it doesn't exist already and copy the dynamic library to it.
    create_dir_all(&godot_project_bin_path)
        .expect("Unable to create bin directory in the Godot project");

    copy_file_to_location(&dynamic_library_file_path, &godot_project_bin_path);

    // Let the user know that the build for the target is complete.
    log_success_to_console(&format!("build for target {} complete.", &target));
}

/// Runs the cargo build command in the library directory to build the dynamic
/// library for the native target.
///
/// # Arguments
///
/// `is_release` - Indicates whether the build is a release build or not.
fn run_cargo_build_command(is_release: bool) {
    let native_target: &str = env!("TARGET");

    let mut cargo_build_command = Command::new("cargo");
    cargo_build_command
        .arg("build")
        .arg("--target")
        .arg(&native_target);

    if is_release {
        cargo_build_command.arg("--release");
    }

    cargo_build_command
        .status()
        .expect("Unable to run cargo build while building the library.");
}

/// Runs the cross build command in the library directory to build the dynamic
/// library for the specified target.
///
/// # Arguments
///
/// `target` - The target to run the build for.
/// `is_release` - Indicates whether the build is a release build or not.
fn run_cross_build_command(target: &str, is_release: bool) {
    let mut cross_build_command = Command::new("cross");
    cross_build_command
        .arg("build")
        .arg("--target")
        .arg(&target);

    if is_release {
        cross_build_command.arg("--release");
    }

    cross_build_command
        .status()
        .expect("Unable to run cargo build while building the library.");
}
