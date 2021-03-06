use chrono::offset::Local;
use convert_case::{Case, Casing};
use notify::{op, raw_watcher, RawEvent, RecursiveMode, Watcher};
use std::fs::create_dir_all;
use std::path::Path;
use std::process::Command;
use std::sync::mpsc::channel;

use crate::config_utils::{get_config_as_object, Config};
use crate::file_utils::copy_file_to_location;
use crate::log_utils::{
    log_error_to_console, log_info_to_console, log_success_to_console, log_version,
};

/// Runs the command to build the library and then copies over the dynamic
/// libraries to the Godot project.
///
/// `is_release` - Indicates whether the build is a release build or not.
/// `build_all_platforms` - Indicates whether all platforms should be built or just the native one.
pub fn build_library(is_release: bool, build_all_platforms: bool) {
    log_version();
    log_info_to_console("[build] build starting...");

    let current_dir = std::env::current_dir().expect("[build] Unable to get current directory.");
    let parent_dir = current_dir
        .parent()
        .expect("[build] Unable to get parent directory.");

    let config = get_config_as_object();
    let library_name_snake_case = &config.name.to_case(Case::Snake);

    // Build for the native platform by default.
    let native_platform = std::env::consts::OS.to_lowercase();
    build_for_platform(
        parent_dir,
        &config,
        &library_name_snake_case,
        &native_platform,
        is_release,
    );

    // Build for all platforms if the flag is passed.
    if build_all_platforms {
        for platform in &config.platforms {
            build_for_platform(
                parent_dir,
                &config,
                &library_name_snake_case,
                &platform.to_lowercase(),
                is_release,
            );
        }
    }

    // Let the user know that the build is complete.
    log_success_to_console("[build] build complete");
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

    let dt = Local::now();
    let current_datetime_formatted = dt.format("%Y-%m-%d %H:%M:%S").to_string();

    // After the build we want to show a message letting the user know that the
    // build has finished and is waiting for futher changes before rebuilding.
    log_info_to_console("");
    log_info_to_console(&format!(
        "[{}] {}",
        current_datetime_formatted, "waiting for changes..."
    ));
}

/// Builds the library for the specified platform. If the platform is not the
/// user's native platform, then the `cross` command will be used.
///
/// # Arguments
///
/// `parent_dir` - The path to the parent directory of the library.
/// `config` - The library configuration.
/// `library_name_snake_case` - The snake case version of the library name.
/// `platform` - The platform to build for.
/// `is_release` - Indicates whether the build is a release build or not.
fn build_for_platform(
    parent_dir: &Path,
    config: &Config,
    library_name_snake_case: &str,
    platform: &str,
    is_release: bool,
) {
    let native_platform = std::env::consts::OS.to_lowercase();

    // The path to the root directory of the dynamic library.
    let current_dir = std::env::current_dir().expect(&format!(
        "[build] Unable to get the path to the dynamic library for the platform {}",
        &platform
    ));
    let debug_or_release_dir_name = if is_release { "release" } else { "debug" };
    let target = get_platform_toolchain(platform.to_string());
    let dynamic_library_root_dir = Path::new(&current_dir)
        .join("target")
        .join(target)
        .join(&debug_or_release_dir_name);
    // The prefix of the dynamic library file.
    let dynamic_library_file_prefix = if platform == "windows" { "" } else { "lib" };

    // The extension of the dynamic library file.
    let dynamic_library_file_extension = if platform == "windows" {
        "dll"
    } else if platform == "macos" {
        "dylib"
    } else {
        "so"
    };

    // The name of the dynamic library file putting together the prefix, the
    // file name, and the extension.
    let dynamic_library_file_name = format!(
        "{}{}.{}",
        &dynamic_library_file_prefix, &library_name_snake_case, &dynamic_library_file_extension
    );

    // The path to the dynamic library file putting together the root dir and
    // the name of the file.
    let dynamic_library_file_path = dynamic_library_root_dir.join(&dynamic_library_file_name);

    log_info_to_console(&format!("[build] building library for {}", &platform));

    if platform == native_platform {
        // If the platform to build for is the user's native platform, then we
        // can use the cargo build command.
        run_cargo_build_command(is_release);
    } else {
        // Otherwise we have to use the `cross` command to build the library
        // for another platform.
        run_cross_build_command(target, is_release);
    }

    // The path to directory in the Godot project where the dynamic library
    // will be copied to.
    let godot_project_bin_path = if config.is_plugin {
        parent_dir
            .join(&config.godot_project_dir_name)
            .join("addons")
            .join(&library_name_snake_case)
            .join("gdnative")
            .join("bin")
            .join(&platform)
    } else {
        parent_dir
            .join(&config.godot_project_dir_name)
            .join("gdnative")
            .join("bin")
            .join(&platform)
    };

    // Make sure that the directory to the path above exists so that we can
    // copy the dynamic library to it.
    create_dir_all(&godot_project_bin_path)
        .expect("[build] Unable to create the bin directory in the Godot project.");

    // Copy the dynamic library from the library to the Godot project.
    copy_file_to_location(&dynamic_library_file_path, &godot_project_bin_path);

    log_success_to_console(&format!(
        "[build] build for platform {} complete.",
        &platform
    ));
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

/// Returns the target to build for depending on the platform.
///
/// # Arguments
///
/// `platform` - The platform to build for.
fn get_platform_toolchain(platform: String) -> &'static str {
    match platform.to_lowercase().as_str() {
        "android.arm" => return "aarch64-linux-android",
        "android" => return "x86_64-linux-android",
        "windows" => return "x86_64-pc-windows-gnu",
        "linux" => return "x86_64-unknown-linux-gnu",
        "macos" => return "x86_64-apple-darwin",
        _ => return "",
    }
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
