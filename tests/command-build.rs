use assert_cmd::prelude::*;

use std::env::set_current_dir;
use std::error::Error;
use std::path::Path;
use std::process::Command;

mod test_utilities;
use test_utilities::{cleanup_test_files, init_test, BUILD_FILE_NAME};

#[test]
fn build_should_create_dynamic_library_file_in_godot_project_bin_directory(
) -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot-rust-cli")?;
  cmd.arg("new").arg("platformer_modules").arg("platformer");

  cmd.assert().success();

  set_current_dir("platformer_modules").expect("Unable to change to library directory");
  Command::new("cargo")
    .arg("run")
    .arg("--manifest-path=../../Cargo.toml")
    .arg("create")
    .arg("Player")
    .output()
    .expect("Unable to execute cargo run");
  Command::new("cargo")
    .arg("run")
    .arg("--manifest-path=../../Cargo.toml")
    .arg("build")
    .output()
    .expect("Unable to execute cargo run");

  let build_file_name = format!("../platformer/bin/{}", BUILD_FILE_NAME);
  let build_file_path = Path::new(&build_file_name);

  assert_eq!(build_file_path.exists(), true);

  set_current_dir("../").expect("Unable to change to parent directory");

  cleanup_test_files();

  Ok(())
}
