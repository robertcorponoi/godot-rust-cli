use std::env::set_current_dir;
use std::fs::{read_to_string, write};
use std::path::PathBuf;
use std::process::{exit, Command};

use convert_case::{Case, Casing};

use crate::command_build::build_library;
use crate::config_utils::create_initial_config;
use crate::definitions::CargoToml;
use crate::file_utils::write_and_fmt;
use crate::log_utils::{log_styled_message_to_console, ConsoleColors};
use crate::path_utils::get_absolute_path;

/// Creates the library used to manage Rust modules.
///
/// # Arguments
///
/// `name` - The name of the library.
/// `godot_project_dir` - The relative path to the directory of the Godot project that this library of modules is for.
/// `skip_build` - Indicates whether the build should be skipped after creating the library or not.
pub fn create_library(name: &str, godot_project_dir: PathBuf, skip_build: bool) {
    log_styled_message_to_console("Creating library", ConsoleColors::WHITE);

    // Normalize the library name so that we can be consistent.
    let library_name_normalized = name.to_case(Case::Snake);

    // Get the absolute path to the library to use in file operations.
    let library_absolute_path = get_absolute_path(&PathBuf::from(&name));

    // Get the absolute path to the Godot project to use in file operations.
    let godot_project_absolute_path = get_absolute_path(&godot_project_dir);

    // Check to see if the library already exists and if the Godot project is
    // valid before proceeding.
    check_if_library_already_exists(library_absolute_path);
    check_if_godot_project_valid(&godot_project_absolute_path);

    create_cargo_library(&library_name_normalized);

    // Change to the library directory so that we can work with the Cargo.toml
    // and set up our dependencies.
    set_current_dir(&library_name_normalized).expect("Unable to change to library directory");

    create_library_cargo_toml();

    create_initial_config(&godot_project_absolute_path);
    create_initial_lib_file();

    create_rust_modules_dir_in_godot(&godot_project_absolute_path);
    create_gdnlib_in_godot(&library_name_normalized, &godot_project_absolute_path);

    log_styled_message_to_console(
        "running initial build to generate Godot project structure",
        ConsoleColors::CYAN,
    );
    if !skip_build {
        build_library();
    }

    log_styled_message_to_console("library created", ConsoleColors::GREEN);
}

/// Creates the `Cargo.toml` file for the library.
fn create_library_cargo_toml() {
    // Get the base Cargo.toml contents of the library.
    let library_cargo_toml_string =
        read_to_string("Cargo.toml").expect("Unable to read library's Cargo.toml file");

    // Add the necessary dependencies to the base contents.
    let new_library_cargo_toml: CargoToml = toml::from_str(&library_cargo_toml_string)
        .expect("Unable to parse the library's Cargo.toml file");

    // Turn the new contents of the library's Cargo.toml into a string so that we
    // can write it back to the library. We also need to normalize some things here
    // because when we turn the Cargo toml contents to a string, extra symbols get
    // added.
    let new_library_cargo_toml_string = toml::to_string(&new_library_cargo_toml)
        .expect("Unable to convert the library's new Cargo.toml to a string")
        .replace("\\", "")
        .replace("\"{", "{")
        .replace("}\"", "}");

    // Next we overwrite the contents of the Cargo.toml file with our contents.
    write("Cargo.toml", new_library_cargo_toml_string)
        .expect("Unable to update contents of the library's Cargo.toml file");
}

/// Checks to see if a directory with the name of the library to create already
/// exists and if it does, then we log it to the console and exit early.
///
/// `library_absolute_path` - The absolute path to the library directory to create.
fn check_if_library_already_exists(library_absolute_path: PathBuf) {
    if library_absolute_path.exists() {
        // If there's already a directory with the library name then we print an
        // error to the console and exit early.
        log_styled_message_to_console(
            "Cannot create library, directory with the same name already exists",
            ConsoleColors::RED,
        );
        exit(1);
    }
}

/// Checks to see if the Godot project is valid by checking whether it has a
/// project.godot file or not.
fn check_if_godot_project_valid(godot_project_absolute_path: &PathBuf) {
    if !godot_project_absolute_path.join("project.godot").exists() {
        // If there's not a project.godot file at the root of the provided Godot
        // project directory then we print an error to the console and exit early.
        log_styled_message_to_console(
            "The Godot project dir provided is not valid",
            ConsoleColors::RED,
        );
        exit(1);
    }
}

/// Creates the library using the `cargo new --lib` command.
///
/// `library_name` - The name of the library to pass to the `cargo new` command.
fn create_cargo_library(library_name: &String) {
    match Command::new("cargo")
        .arg("new")
        .arg(&library_name)
        .arg("--lib")
        .output()
    {
        Ok(_v) => (),
        Err(e) => {
            log_styled_message_to_console(&e.to_string(), ConsoleColors::RED);
            exit(1);
        }
    }
}

/// Creates the `rust_modules` directory within the Godot project.
///
/// `godot_project_path` - The path to the Godot project.
fn create_rust_modules_dir_in_godot(godot_project_path: &PathBuf) {
    match std::fs::create_dir_all(&godot_project_path.join("rust_modules")) {
        Ok(_) => (),
        Err(e) => {
            // If there was a problem creating the directory then we print the error
            // to the console and exit early.
            log_styled_message_to_console(&e.to_string(), ConsoleColors::RED);
            exit(1);
        }
    }
}

/// Creates the gdnlib file in the Godot project directory.
///
/// # Arguments
///
/// `library_name` - The name of the library to create.
/// `godot_project_dir` - The absolute path to the Godot project.
fn create_gdnlib_in_godot(library_name: &str, godot_absolute_path: &PathBuf) {
    let gdnlib_template = include_str!("../templates/gdnlib.txt");

    let gdnlib_with_library_name = gdnlib_template.replace("LIBRARY_NAME", &library_name);
    let gdnlib_filename = format!("{}.gdnlib", &library_name);

    write(
        &godot_absolute_path.join(gdnlib_filename),
        gdnlib_with_library_name,
    )
    .expect("Unable to create gdnlib file");
}

/// Creates the initial `lib.rs` file in the library directory.
fn create_initial_lib_file() {
    let lib_template = include_str!("../templates/lib.rs");
    write_and_fmt("src/lib.rs", lib_template).expect("Unable to create the initial lib.rs file");
}
