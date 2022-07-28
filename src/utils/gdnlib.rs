use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::write;
use std::path::PathBuf;
use std::process::exit;

use crate::config_utils::Config;
use crate::log_utils::log_error_to_console;
use convert_case::{Case, Casing};

/// The structure of the gdnlib file.
#[derive(Debug, Serialize, Deserialize)]
pub struct Gdnlib {
    general: GdnlibGeneral,
    entry: HashMap<String, String>,
    dependencies: HashMap<String, Vec<String>>,
}

/// The structure of the general section of the gdnlib file.
#[derive(Debug, Serialize, Deserialize)]
pub struct GdnlibGeneral {
    pub singleton: bool,
    pub load_once: bool,
    pub symbol_prefix: String,
    pub reloadable: bool,
}

/// Returns the path to the gdnlib file from the root of the library.
///
/// `config` - The configuration object.
/// `godot_project_absolute_path` - The absolute path to the Godot project.
pub fn get_path_to_gdnlib_file(config: &Config, godot_project_absolute_path: PathBuf) -> PathBuf {
    let library_name_snake_case = &config.name.to_case(Case::Snake);
    let gdnlib_directory: PathBuf = if config.is_plugin {
        godot_project_absolute_path
            .join("addons")
            .join(library_name_snake_case)
            .join("gdnative")
    } else {
        godot_project_absolute_path.join("gdnative")
    };

    std::fs::create_dir_all(&gdnlib_directory).expect("Unable to create dir for gdnlib file.");

    return gdnlib_directory.join(format!("{}.gdnlib", library_name_snake_case));
}

/// Creates the initial gdnlib and saves it to the Godot project.
///
/// # Arguments
///
/// `config` - The configuration object.
/// `godot_project_absolute_path` - The absolute path to the Godot project.
pub fn create_initial_gdnlib(config: &Config, godot_project_absolute_path: PathBuf) -> Gdnlib {
    let library_name_snake_case = &config.name.to_case(Case::Snake);

    let entries_and_dependencies =
        get_entries_and_dependencies_to_add_to_gdnlib(library_name_snake_case, config.is_plugin);

    let gdnlib_general = GdnlibGeneral {
        singleton: false,
        load_once: true,
        symbol_prefix: "godot_".to_string(),
        reloadable: true,
    };
    let mut gdnlib = Gdnlib {
        general: gdnlib_general,
        entry: entries_and_dependencies.0,
        dependencies: entries_and_dependencies.1,
    };

    save_gdnlib_to_file(config, &mut gdnlib, godot_project_absolute_path);

    return gdnlib;
}

/// Saves the gdnlib file to the Godot project directory.
///
/// # Arguments
///
/// `config` - The configuration object.
/// `gdnlib_file` - The gdnlib object to save.
/// `godot_project_absolute_path` - The absolute path to the Godot project.
pub fn save_gdnlib_to_file(
    config: &Config,
    gdnlib: &mut Gdnlib,
    godot_project_absolute_path: PathBuf,
) {
    let gdnlib_file_path = get_path_to_gdnlib_file(config, godot_project_absolute_path);
    let gdnlib_as_string =
        toml::to_string_pretty(&gdnlib).expect("Unable to convert gdnlib to string.");
    match write(&gdnlib_file_path, gdnlib_as_string.replace("'", "\"")) {
        Ok(_) => (),
        Err(e) => {
            log_error_to_console(&e.to_string());
            exit(1);
        }
    }
}

/// Returns the entries and dependencies that need to be added to the gdnlib
/// object.
///
/// # Arguments
///
/// `library_name_snake_case` - The snake case version of the library name.
/// `is_plugin` - Indicates whether the library is for a plugin or not.
fn get_entries_and_dependencies_to_add_to_gdnlib(
    library_name_snake_case: &str,
    is_plugin: bool,
) -> (HashMap<String, String>, HashMap<String, Vec<String>>) {
    let base_path = if is_plugin {
        format!("res://addons/{}", library_name_snake_case)
    } else {
        "res:/".to_owned()
    };

    let osx_bin_path = format!(
        "{}/gdnative/bin/macos/lib{}.dylib",
        base_path, library_name_snake_case
    );
    let windows_bin_path = format!(
        "{}/gdnative/bin/windows/{}.dll",
        base_path, library_name_snake_case
    );
    let linux_bin_path = format!(
        "{}/gdnative/bin/linux/lib{}.so",
        base_path, library_name_snake_case
    );

    let mut entries: HashMap<String, String> = HashMap::new();
    entries.insert("OSX.64".to_owned(), osx_bin_path);
    entries.insert("Windows.64".to_owned(), windows_bin_path);
    entries.insert("X11.64".to_owned(), linux_bin_path);

    // Entries for the Android OS.
    let android_arm_bin_path = format!(
        "{}/gdnative/bin/android/aarch64-linux-android/lib{}.so",
        base_path, library_name_snake_case
    );
    let android_64_bin_path = format!(
        "{}/gdnative/bin/android/x86_64-linux-android/lib{}.so",
        base_path, library_name_snake_case
    );
    entries.insert("Android.arm64-v8a".to_owned(), android_arm_bin_path);
    entries.insert("Android.x86_64".to_owned(), android_64_bin_path);

    let mut deps: HashMap<String, Vec<String>> = HashMap::new();
    deps.insert("OSX.64".to_owned(), vec![]);
    deps.insert("Windows.64".to_owned(), vec![]);
    deps.insert("X11.64".to_owned(), vec![]);
    deps.insert("Android.arm64-v8a".to_owned(), vec![]);
    deps.insert("Android.x86_64".to_owned(), vec![]);

    return (entries, deps);
}
