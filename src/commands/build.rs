use chrono::offset::Local;
use convert_case::{Case, Casing};
use notify::{op, raw_watcher, RawEvent, RecursiveMode, Watcher};
use std::fs::create_dir_all;
use std::process::Command;
use std::sync::mpsc::channel;

use crate::config_utils::get_config_as_object;
use crate::dynamic_library_utils::get_dynamic_library_ext;
use crate::file_utils::copy_file_to_location;
use crate::log_utils::{log_styled_message_to_console, log_version, ConsoleColors};
use crate::path_utils::{get_dynamic_libraries_path, get_library_name_from_path};
use crate::time_utils::get_current_datetime_formatted;

/// Builds the library to generate the dynamic libraries needed to run the
/// modules in the Godot project.
pub fn build_library() {
    // Get the parent directory since it contains the library and the Godot
    // project directories.
    let current_dir = std::env::current_dir().expect("Unable to get current directory");
    let parent_dir = current_dir.parent().expect("Unable to get parent dir");

    // The configuration object.
    let config = get_config_as_object();

    // The name of the library.
    let lib_name = get_library_name_from_path();

    // The path to the dynamic libraries in the library. This varies on Unix
    // and Windows systems.
    let dynamic_libraries_path = get_dynamic_libraries_path();

    // Get the extention of the dynamic library generated on the OS that the
    // command is being run on.
    let dynamic_library_ext = get_dynamic_library_ext();

    // If the platform that the library is being built on is not windows, then we
    // need to add an extra "lib" part before the dynamic library file name.
    let dynamic_library_extra = if cfg!(windows) { "" } else { "lib" };

    // Join all of the information together to get the path to the dynamic
    // library file name.
    let dynamic_library_file_name = format!(
        "{}{}.{}",
        dynamic_library_extra,
        lib_name.to_case(Case::Snake),
        dynamic_library_ext
    );
    let dynamic_library_file = dynamic_libraries_path.join(dynamic_library_file_name);
    // The path to the Godot project.
    let godot_project_dir = parent_dir.join(&config.godot_project_name);

    log_version();
    log_styled_message_to_console("building...", ConsoleColors::CYAN);

    // Run the actual `cargo build` command to build the library.
    run_cargo_build();

    // The path to where the dynamic libraries should be stored in the Godot
    // project directory.
    let bin_path = if config.is_plugin {
        godot_project_dir
            .join("addons")
            .join(&config.name.to_case(Case::Snake))
            .join("bin")
    } else {
        parent_dir.join(&config.godot_project_name).join("bin")
    };

    // Create the `bin` folder in the Godot project if it doesn't already exist.
    create_dir_all(&bin_path).expect("Unable to create bin directory in the Godot project");

    copy_file_to_location(&dynamic_library_file, &bin_path);

    log_styled_message_to_console("Build complete", ConsoleColors::GREEN);
}

/// Watches the `src` directory in the library for changes and runs the build
/// command automatically.
pub fn build_library_and_watch() {
    let (tx, rx) = channel();

    // Run the initial build.
    build_with_timestamp();

    let mut last_checked = Local::now();
    let mut watcher = raw_watcher(tx).expect("Unable to create watcher");
    let current_dir = std::env::current_dir().expect("Unable to get current directory");

    watcher
        .watch(current_dir.join("src"), RecursiveMode::Recursive)
        .expect("Unable to watch src directory");
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
                        build_with_timestamp();
                    }
                    last_checked = Local::now();
                }
            }
            Ok(_event) => log_styled_message_to_console("broken event", ConsoleColors::RED),
            Err(e) => log_styled_message_to_console(&e.to_string(), ConsoleColors::RED),
        }
    }
}

/// Runs the `build` command and logs the time that the build as started in the
/// format of `YYYY-MM-DD HH:MM:SS`.
pub fn build_with_timestamp() {
    build_library();

    // After the build we want to show a message letting the user know that the
    // build has finished and is waiting for futher changes before rebuilding.
    log_styled_message_to_console("", ConsoleColors::WHITE);
    log_styled_message_to_console(
        &format!(
            "[{}] {}",
            get_current_datetime_formatted(),
            "waiting for changes..."
        ),
        ConsoleColors::WHITE,
    );
}

/// Runs the `cargo build` command in the library directory to build the
/// dynamic libraries for the library.
fn run_cargo_build() {
    let cargo_build = Command::new("cargo")
        .arg("build")
        .status()
        .expect("Unable to run cargo build");

    if !cargo_build.success() {
        log_styled_message_to_console("Build failed, please try again", ConsoleColors::RED);
    }
}
