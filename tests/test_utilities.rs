use std::fs::{remove_dir_all, remove_file};
use std::path::Path;

/// If the tests are being run on windows, then the build file is a dll file.
#[cfg(target_os = "windows")]
#[allow(dead_code)]
pub const BUILD_FILE_NAME: &str = "platformer_modules.dll";

/// If the tests are being run on linux, then the build file is a libx.so file.
#[cfg(target_os = "linux")]
#[allow(dead_code)]
pub const BUILD_FILE_NAME: &str = "libplatformer_modules.so";

/// If the tests are being run on macos, then the build file is a libx.dylib file.
#[cfg(target_os = "macos")]
#[allow(dead_code)]
pub const BUILD_FILE_NAME: &str = "libplatformer_modules.dylib";

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
      remove_file("platformer/libplatformer_modules.dylib").expect("Unable to remove macos file");
    }
  }

  if Path::new("platformer/addons").exists() {
    remove_dir_all("platformer/addons").expect("Unable to remove plugin addons dir");
  }
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
