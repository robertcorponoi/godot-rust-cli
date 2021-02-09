use assert_cmd::prelude::*;

use std::env::set_current_dir;
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use std::process::Command;

mod test_utilities;
use test_utilities::{cleanup_test_files, init_test};

#[test]
fn destroy_remove_module_from_library_lib_file_and_godot_project_rust_modules(
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
    .arg("destroy")
    .arg("Player")
    .output()
    .expect("Unable to execute cargo run");

  let lib_file = read_to_string("src/lib.rs").expect("Unable to read lib file");
  let lib_file_split = lib_file.split("\n").collect::<Vec<&str>>();

  let config_file = read_to_string("project.toml").expect("Unable to read config file");
  let config_split = config_file.split("\n").collect::<Vec<&str>>();

  let mod_file_path = Path::new("src/player.rs");

  let player_gdns_file = Path::new("../platformer/rust_modules/player.gdns");

  assert_eq!(lib_file_split[0], "use gdnative::prelude::*;");
  assert_eq!(lib_file_split[1], "");
  assert_eq!(lib_file_split[2], "fn init(handle: InitHandle) {}");
  assert_eq!(lib_file_split[3], "");
  assert_eq!(lib_file_split[4], "godot_init!(init);");
  assert_eq!(config_split[1], "modules = []");
  assert_eq!(mod_file_path.exists(), false);
  assert_eq!(player_gdns_file.exists(), false);

  set_current_dir("../").expect("Unable to change to parent directory");

  cleanup_test_files();

  Ok(())
}

#[test]
fn destroy_create_five_modules_remove_two_modules() -> Result<(), Box<dyn Error>> {
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
    .arg("create")
    .arg("MainScene")
    .output()
    .expect("Unable to execute cargo run");
  Command::new("cargo")
    .arg("run")
    .arg("--manifest-path=../../Cargo.toml")
    .arg("create")
    .arg("Ship")
    .output()
    .expect("Unable to execute cargo run");
  Command::new("cargo")
    .arg("run")
    .arg("--manifest-path=../../Cargo.toml")
    .arg("create")
    .arg("Vehicle")
    .output()
    .expect("Unable to execute cargo run");
  Command::new("cargo")
    .arg("run")
    .arg("--manifest-path=../../Cargo.toml")
    .arg("create")
    .arg("Enemy")
    .output()
    .expect("Unable to execute cargo run");
  Command::new("cargo")
    .arg("run")
    .arg("--manifest-path=../../Cargo.toml")
    .arg("destroy")
    .arg("Ship")
    .output()
    .expect("Unable to execute cargo run");
  Command::new("cargo")
    .arg("run")
    .arg("--manifest-path=../../Cargo.toml")
    .arg("destroy")
    .arg("Enemy")
    .output()
    .expect("Unable to execute cargo run");

  let lib_file = read_to_string("src/lib.rs").expect("Unable to read lib file");
  let lib_file_split = lib_file.split("\n").collect::<Vec<&str>>();

  let config_file = read_to_string("project.toml").expect("Unable to read config file");
  let config_split = config_file.split("\n").collect::<Vec<&str>>();

  let player_mod_file_path = Path::new("src/player.rs");
  let main_scene_mod_file_path = Path::new("src/main_scene.rs");
  let ship_mod_file_path = Path::new("src/ship.rs");
  let vehicle_mod_file_path = Path::new("src/vehicle.rs");
  let enemy_mod_file_path = Path::new("src/enemy.rs");

  let player_gdns_file = Path::new("../platformer/rust_modules/player.gdns");
  let main_scene_gdns_file = Path::new("../platformer/rust_modules/main_scene.gdns");
  let ship_gdns_file = Path::new("../platformer/rust_modules/ship.gdns");
  let vehicle_gdns_file = Path::new("../platformer/rust_modules/vehicle.gdns");
  let enemy_gdns_file = Path::new("../platformer/rust_modules/enemy.gdns");

  assert_eq!(lib_file_split[0], "mod main_scene;");
  assert_eq!(lib_file_split[1], "mod player;");
  assert_eq!(lib_file_split[2], "mod vehicle;");
  assert_eq!(lib_file_split[3], "use gdnative::prelude::*;");
  assert_eq!(lib_file_split[4], "");
  assert_eq!(lib_file_split[5], "fn init(handle: InitHandle) {");
  assert_eq!(
    lib_file_split[6].trim(),
    "handle.add_class::<player::Player>();"
  );
  assert_eq!(
    lib_file_split[7].trim(),
    "handle.add_class::<vehicle::Vehicle>();"
  );
  assert_eq!(
    lib_file_split[8].trim(),
    "handle.add_class::<main_scene::MainScene>();"
  );
  assert_eq!(lib_file_split[9], "}");
  assert_eq!(lib_file_split[10], "");
  assert_eq!(lib_file_split[11], "godot_init!(init);");

  assert_eq!(
    config_split[1],
    "modules = [\"Player\", \"MainScene\", \"Vehicle\"]"
  );

  assert_eq!(player_mod_file_path.exists(), true);
  assert_eq!(main_scene_mod_file_path.exists(), true);
  assert_eq!(vehicle_mod_file_path.exists(), true);
  assert_eq!(enemy_mod_file_path.exists(), false);
  assert_eq!(ship_mod_file_path.exists(), false);

  assert_eq!(player_gdns_file.exists(), true);
  assert_eq!(main_scene_gdns_file.exists(), true);
  assert_eq!(vehicle_gdns_file.exists(), true);
  assert_eq!(enemy_gdns_file.exists(), false);
  assert_eq!(ship_gdns_file.exists(), false);

  set_current_dir("../").expect("Unable to change to parent directory");

  cleanup_test_files();

  Ok(())
}
