use assert_cmd::prelude::*;

use std::env::set_current_dir;
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use std::process::Command;

mod test_utilities;
use test_utilities::{cleanup_test_files, init_test};

#[test]
fn create_add_module_name_as_pascalcase_to_project_toml_modules() -> Result<(), Box<dyn Error>> {
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

  let config = read_to_string("project.toml").expect("Unable to read config");
  let config_split = config.split("\n").collect::<Vec<&str>>();

  assert_eq!(config_split[0], "godot_project_name = \"platformer\"");
  assert_eq!(config_split[1], "modules = [\"Player\"]");

  set_current_dir("../").expect("Unable to change to parent directory");

  cleanup_test_files();

  Ok(())
}

#[test]
fn create_add_module_mod_and_handle_to_to_lib_file() -> Result<(), Box<dyn Error>> {
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

  let lib_file = read_to_string("src/lib.rs").expect("Unable to read lib file");
  let lib_file_split = lib_file.split("\n").collect::<Vec<&str>>();

  assert_eq!(lib_file_split[0], "mod player;");
  assert_eq!(lib_file_split[1], "use gdnative::prelude::*;");
  assert_eq!(lib_file_split[2], "");
  assert_eq!(lib_file_split[3], "fn init(handle: InitHandle) {");
  assert_eq!(
    lib_file_split[4].trim(),
    "handle.add_class::<player::Player>();"
  );
  assert_eq!(lib_file_split[5], "}");
  assert_eq!(lib_file_split[6], "");
  assert_eq!(lib_file_split[7], "godot_init!(init);");

  set_current_dir("../").expect("Unable to change to parent directory");

  cleanup_test_files();

  Ok(())
}

#[test]
fn create_module_has_correct_initial_module_file() -> Result<(), Box<dyn Error>> {
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

  let mod_file = read_to_string("src/player.rs").expect("Unable to read module file");
  let mod_file_split = mod_file.split("\n").collect::<Vec<&str>>();

  assert_eq!(mod_file_split[0].trim(), "use gdnative::api::Node2D;");
  assert_eq!(mod_file_split[1].trim(), "use gdnative::prelude::*;");
  assert_eq!(mod_file_split[2].trim(), "");
  assert_eq!(mod_file_split[3].trim(), "#[inherit(Node2D)]");
  assert_eq!(mod_file_split[4].trim(), "#[derive(NativeClass)]");
  assert_eq!(mod_file_split[5].trim(), "pub struct Player;");
  assert_eq!(mod_file_split[6].trim(), "");
  assert_eq!(mod_file_split[7].trim(), "#[methods]");
  assert_eq!(mod_file_split[8].trim(), "impl Player {");
  assert_eq!(
    mod_file_split[9].trim(),
    "fn new(_owner: &Node2D) -> Self {"
  );
  assert_eq!(mod_file_split[10].trim(), "Player {}");
  assert_eq!(mod_file_split[11].trim(), "}");
  assert_eq!(mod_file_split[12].trim(), "#[export]");
  assert_eq!(
    mod_file_split[13].trim(),
    "fn _ready(&mut self, _owner: &Node2D) {"
  );
  assert_eq!(mod_file_split[14].trim(), "godot_print!(\"Hello world!\");");
  assert_eq!(mod_file_split[15].trim(), "}");
  assert_eq!(mod_file_split[16].trim(), "");
  assert_eq!(mod_file_split[17].trim(), "#[export]");
  assert_eq!(
    mod_file_split[18].trim(),
    "fn _process(&mut self, _owner: &Node2D, _delta: f32) {}"
  );

  set_current_dir("../").expect("Unable to change to parent directory");

  cleanup_test_files();

  Ok(())
}

#[test]
fn create_module_gdns_file_in_godot_project_rust_modules_directory() -> Result<(), Box<dyn Error>> {
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

  let gdns_file_contents =
    read_to_string("../platformer/rust_modules/player.gdns").expect("Unable to read gdns file");
  let gdns_file_contents_split = gdns_file_contents.split("\n").collect::<Vec<&str>>();

  assert_eq!(
    gdns_file_contents_split[0].trim(),
    "[gd_resource type=\"NativeScript\" load_steps=2 format=2]"
  );
  assert_eq!(gdns_file_contents_split[1].trim(), "");
  assert_eq!(
    gdns_file_contents_split[2].trim(),
    "[ext_resource path=\"res://platformer_modules.gdnlib\" type=\"GDNativeLibrary\" id=1]"
  );
  assert_eq!(gdns_file_contents_split[3].trim(), "");
  assert_eq!(gdns_file_contents_split[4].trim(), "[resource]");
  assert_eq!(gdns_file_contents_split[5].trim(), "");
  assert_eq!(
    gdns_file_contents_split[6].trim(),
    "resource_name = \"Player\""
  );
  assert_eq!(
    gdns_file_contents_split[7].trim(),
    "class_name = \"Player\""
  );
  assert_eq!(
    gdns_file_contents_split[8].trim(),
    "library = ExtResource( 1 )"
  );

  set_current_dir("../").expect("Unable to change to parent directory");

  cleanup_test_files();

  Ok(())
}

#[test]
fn create_module_name_casing_normalized() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot-rust-cli")?;
  cmd.arg("new").arg("platformer_modules").arg("platformer");

  cmd.assert().success();

  set_current_dir("platformer_modules").expect("Unable to change to library directory");
  Command::new("cargo")
    .arg("run")
    .arg("--manifest-path=../../Cargo.toml")
    .arg("create")
    .arg("MainScene")
    .output()
    .expect("Unable to execute cargo run");

  let config_file = read_to_string("project.toml").expect("Unable to read config file");
  let mod_file = read_to_string("src/main_scene.rs").expect("Unable to read module file");
  let lib_file = read_to_string("src/lib.rs").expect("Unable to read lib file");

  let mod_file_split = mod_file.split("\n").collect::<Vec<&str>>();
  let config_split = config_file.split("\n").collect::<Vec<&str>>();
  let lib_file_split = lib_file.split("\n").collect::<Vec<&str>>();

  assert_eq!(mod_file_split[5].trim(), "pub struct MainScene;");
  assert_eq!(mod_file_split[8].trim(), "impl MainScene {");
  assert_eq!(mod_file_split[10].trim(), "MainScene {}");

  assert_eq!(config_split[1], "modules = [\"MainScene\"]");

  assert_eq!(lib_file_split[0], "mod main_scene;");
  assert_eq!(
    lib_file_split[4].trim(),
    "handle.add_class::<main_scene::MainScene>();"
  );

  set_current_dir("../").expect("Unable to change to parent directory");

  cleanup_test_files();

  Ok(())
}

#[test]
fn create_multiple_modules_should_have_gdns_files_created() -> Result<(), Box<dyn Error>> {
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

  let player_file_path = Path::new("src/player.rs");
  let main_scene_file_path = Path::new("src/main_scene.rs");
  let gdnlib_file_path = Path::new("../platformer/platformer_modules.gdnlib");
  let player_ns_file_path = Path::new("../platformer/rust_modules/player.gdns");
  let main_scene_ns_file_path = Path::new("../platformer/rust_modules/main_scene.gdns");

  let config_file = read_to_string("project.toml").expect("Unable to read config file");
  let config_split = config_file.split("\n").collect::<Vec<&str>>();

  assert_eq!(player_file_path.exists(), true);
  assert_eq!(main_scene_file_path.exists(), true);
  assert_eq!(gdnlib_file_path.exists(), true);
  assert_eq!(player_ns_file_path.exists(), true);
  assert_eq!(main_scene_ns_file_path.exists(), true);
  assert_eq!(config_split[1], "modules = [\"Player\", \"MainScene\"]");

  set_current_dir("../").expect("Unable to change to parent directory");

  cleanup_test_files();

  Ok(())
}

#[test]
fn create_multiple_modules_should_be_added_to_lib_file() -> Result<(), Box<dyn Error>> {
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

  let lib_file = read_to_string("src/lib.rs").expect("Unable to read lib file");
  let lib_file_split = lib_file.split("\n").collect::<Vec<&str>>();

  assert_eq!(lib_file_split[0], "mod main_scene;");
  assert_eq!(lib_file_split[1], "mod player;");
  assert_eq!(lib_file_split[2], "use gdnative::prelude::*;");
  assert_eq!(lib_file_split[3], "");
  assert_eq!(lib_file_split[4], "fn init(handle: InitHandle) {");
  assert_eq!(
    lib_file_split[5].trim(),
    "handle.add_class::<player::Player>();"
  );
  assert_eq!(
    lib_file_split[6].trim(),
    "handle.add_class::<main_scene::MainScene>();"
  );
  assert_eq!(lib_file_split[7], "}");
  assert_eq!(lib_file_split[8], "");
  assert_eq!(lib_file_split[9], "godot_init!(init);");
  assert_eq!(lib_file_split[10], "");

  set_current_dir("../").expect("Unable to change to parent directory");

  cleanup_test_files();

  Ok(())
}
