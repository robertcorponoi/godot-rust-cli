use assert_cmd::prelude::*;

use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use std::process::Command;

mod test_utilities;
use test_utilities::{cleanup_test_files, init_test};

#[test]
fn new_library_has_correct_cargo_toml() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot-rust-cli")?;
  cmd.arg("new").arg("platformer_modules").arg("platformer");

  cmd.assert().success();

  let cargo_toml =
    read_to_string("platformer_modules/Cargo.toml").expect("Unable to read Cargo.toml");
  let cargo_toml_split = cargo_toml.split("\n").collect::<Vec<&str>>();

  assert_eq!(cargo_toml_split[6], "[lib]");
  assert_eq!(cargo_toml_split[7], "crate-type = [\"cdylib\"]");
  assert_eq!(cargo_toml_split[9], "[dependencies]");
  assert_eq!(cargo_toml_split[10], "gdnative = \"0.9.1\"");

  cleanup_test_files();

  Ok(())
}

#[test]
fn new_godot_project_has_rust_modules_directory() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot-rust-cli")?;
  cmd.arg("new").arg("platformer_modules").arg("platformer");

  cmd.assert().success();

  let rust_modules = Path::new("./platformer/rust_modules");
  assert_eq!(rust_modules.exists(), true);

  cleanup_test_files();

  Ok(())
}

#[test]
fn new_library_name_is_snake_case() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot-rust-cli")?;
  cmd.arg("new").arg("platformer-modules").arg("platformer");

  cmd.assert().success();

  let cargo_toml =
    read_to_string("platformer_modules/Cargo.toml").expect("Unable to read Cargo.toml");

  assert_eq!(cargo_toml.is_empty(), false);

  cleanup_test_files();

  Ok(())
}

#[test]
fn new_library_has_correct_project_toml() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot-rust-cli")?;
  cmd.arg("new").arg("platformer_modules").arg("platformer");

  cmd.assert().success();

  let config = read_to_string("platformer_modules/project.toml").expect("Unable to read config");
  let config_split = config.split("\n").collect::<Vec<&str>>();

  let gdnlib_path = Path::new("platformer/platformer_modules.gdnlib");

  assert_eq!(config_split[0], "godot_project_name = \"platformer\"");
  assert_eq!(config_split[1], "modules = []");

  assert_eq!(gdnlib_path.exists(), true);

  cleanup_test_files();

  Ok(())
}

#[test]
fn new_has_initial_lib_file() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot-rust-cli")?;
  cmd.arg("new").arg("platformer_modules").arg("platformer");

  cmd.assert().success();

  let lib = read_to_string("platformer_modules/src/lib.rs").expect("Unable to read lib file");
  let lib_split = lib.split("\n").collect::<Vec<&str>>();

  assert_eq!(lib_split[0], "use gdnative::prelude::*;");
  assert_eq!(lib_split[1], "");
  assert_eq!(lib_split[2], "fn init(handle: InitHandle) {}");
  assert_eq!(lib_split[3], "");
  assert_eq!(lib_split[4], "godot_init!(init);");

  cleanup_test_files();

  Ok(())
}

#[test]
fn new_godot_project_has_correct_gdnlib_file() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot-rust-cli")?;
  cmd.arg("new").arg("platformer_modules").arg("platformer");

  cmd.assert().success();

  let gdnlib =
    read_to_string("platformer/platformer_modules.gdnlib").expect("Unable to read gdnlib");
  let gdnlib_split = gdnlib.split("\n").collect::<Vec<&str>>();

  assert_eq!(gdnlib_split[0].trim(), "[general]");
  assert_eq!(gdnlib_split[1].trim(), "");
  assert_eq!(gdnlib_split[2].trim(), "singleton=false");
  assert_eq!(gdnlib_split[3].trim(), "load_once=true");
  assert_eq!(gdnlib_split[4].trim(), "symbol_prefix=\"godot_\"");
  assert_eq!(gdnlib_split[5].trim(), "reloadable=true");
  assert_eq!(gdnlib_split[6].trim(), "");
  assert_eq!(gdnlib_split[7].trim(), "[entry]");
  assert_eq!(gdnlib_split[8].trim(), "");
  assert_eq!(
    gdnlib_split[9].trim(),
    "OSX.64=\"res://bin/libplatformer_modules.dylib\""
  );
  assert_eq!(
    gdnlib_split[10].trim(),
    "Windows.64=\"res://bin/platformer_modules.dll\""
  );
  assert_eq!(
    gdnlib_split[11].trim(),
    "X11.64=\"res://bin/libplatformer_modules.so\""
  );
  assert_eq!(gdnlib_split[12].trim(), "");
  assert_eq!(gdnlib_split[13].trim(), "[dependencies]");
  assert_eq!(gdnlib_split[14].trim(), "");
  assert_eq!(gdnlib_split[15].trim(), "OSX.64=[  ]");
  assert_eq!(gdnlib_split[16].trim(), "Windows.64=[  ]");
  assert_eq!(gdnlib_split[17].trim(), "X11.64=[  ]");

  cleanup_test_files();

  Ok(())
}
