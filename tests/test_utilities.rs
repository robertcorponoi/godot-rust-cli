use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{remove_dir_all, remove_file};
use std::path::Path;
use std::process::Command;

/// The structure of the gdnlib file.
#[derive(Debug, Serialize, Deserialize)]
pub struct Gdnlib {
    pub general: GdnlibGeneral,
    pub entry: HashMap<String, String>,
    pub dependencies: HashMap<String, Vec<String>>,
}

/// The structure of the general section of the gdnlib file.
#[derive(Debug, Serialize, Deserialize)]
pub struct GdnlibGeneral {
    pub singleton: bool,
    pub load_once: bool,
    pub symbol_prefix: String,
    pub reloadable: bool,
}

/// Some tests need to change directory to run correctly so this function is
/// called after those tests to go back to the tests directory that every
/// function expects to be in.
#[allow(dead_code)]
pub fn ensure_correct_dir() {
    let current_dir = std::env::current_dir().unwrap();
    let current_dir_basename = current_dir.file_stem().unwrap();

    if current_dir_basename != "tests" {
        std::env::set_current_dir("tests").expect("Unable to change to tests directory");
    }
}

/// Creates a folder with a project.godot file at the root to simulate the tests
/// running on an actual Godot project.
#[allow(dead_code)]
pub fn create_godot_project() {
    std::fs::create_dir("platformer").expect("Unable to create Godot project directory");
    std::fs::File::create("platformer/project.godot").expect("Unable to create godot.project file");
}

/// Since tests create folders and files we need to remove them before running
/// the next tests.
#[allow(dead_code)]
pub fn cleanup_test_files() {
    if Path::new("platformer").exists() {
        remove_dir_all("platformer").expect("Unable to remove Godot project dir");
    }

    if Path::new("platformer_modules").exists() {
        remove_dir_all("platformer_modules").expect("Unable to remove library dir");
    }

    if Path::new("directory_browser").exists() {
        remove_dir_all("directory_browser").expect("Unable to remove plugin dir");
    }

    if Path::new("platformer/platformer_modules.gdnlib").exists() {
        remove_file("platformer/platformer_modules.gdnlib").expect("Unable to remove gdnlib file");
        if Path::new("platformer/platformer_modules.dll").exists() {
            remove_file("platformer/platformer_modules.dll").expect("Unable to remove dll file");
        }
        if Path::new("platformer/libplatformer_modules.so").exists() {
            remove_file("platformer/libplatformer_modules.so").expect("Unable to remove so file");
        }
        if Path::new("platformer/libplatformer_modules.dylib").exists() {
            remove_file("platformer/libplatformer_modules.dylib")
                .expect("Unable to remove macos file");
        }
    }

    if Path::new("platformer/addons").exists() {
        remove_dir_all("platformer/addons").expect("Unable to remove plugin addons dir");
    }
}

/// Some tests create docker images so we want to remove them after the tests.
#[allow(dead_code)]
pub fn cleanup_docker_images() {
    let mut cmd_remove_windows_docker_image = Command::new("docker");
    cmd_remove_windows_docker_image
        .arg("image")
        .arg("rmi")
        .arg("rustembedded/cross:x86_64-pc-windows-gnu");
    cmd_remove_windows_docker_image.status().unwrap();

    let mut cmd_remove_custom_windows_docker_image = Command::new("docker");
    cmd_remove_custom_windows_docker_image
        .arg("image")
        .arg("rmi")
        .arg("godot-rust-cli-platform-windows:v1");
    cmd_remove_custom_windows_docker_image.status().unwrap();
}

/// Returns the path of the specified file.
///
/// # Arguments
///
/// `file_to_find` - The name of the file to find.
#[allow(dead_code)]
fn find_file(file_to_find: String) -> std::path::PathBuf {
    let mut exists = false;
    let current_dir = std::env::current_dir().expect("Unable to get current directory");
    let mut dir_to_check = std::path::Path::new(&current_dir);
    let mut iterations = 0;

    while !exists && iterations <= 10 {
        let temp_path = std::path::Path::new(&dir_to_check).join(&file_to_find);
        exists = temp_path.exists();

        if !exists {
            iterations += 1;
            dir_to_check = dir_to_check
                .parent()
                .expect("Unable to get parent directory");
        }
    }

    return dir_to_check.to_owned();
}

/// Runs before each test.
#[allow(dead_code)]
pub fn init_test() {
    ensure_correct_dir();
    cleanup_test_files();
    create_godot_project();
}
