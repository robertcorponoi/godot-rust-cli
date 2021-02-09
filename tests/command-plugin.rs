use assert_cmd::prelude::*;

use std::env::set_current_dir;
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use std::process::Command;

mod test_utilities;
use test_utilities::{cleanup_test_files, init_test};

#[test]
fn plugin_add_name_to_library_project_toml() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot-rust-cli")?;
  cmd.arg("new").arg("platformer_modules").arg("platformer");

  cmd.assert().success();

  set_current_dir("platformer_modules").expect("Unable to change to library directory");
  Command::new("cargo")
    .arg("run")
    .arg("--manifest-path=../../Cargo.toml")
    .arg("plugin")
    .arg("Directory Browser")
    .output()
    .expect("Unable to execute cargo run");

  let config = read_to_string("project.toml").expect("Unable to read config");
  let config_split = config.split("\n").collect::<Vec<&str>>();

  assert_eq!(config_split[0], "godot_project_name = \"platformer\"");
  assert_eq!(config_split[1], "modules = [\"DirectoryBrowser\"]");

  set_current_dir("../").expect("Unable to change to parent directory");

  cleanup_test_files();

  Ok(())
}

#[test]
fn plugin_has_correct_plugin_cfg_in_godot_project() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot-rust-cli")?;
  cmd.arg("new").arg("platformer_modules").arg("platformer");

  cmd.assert().success();

  set_current_dir("platformer_modules").expect("Unable to change to library directory");
  Command::new("cargo")
    .arg("run")
    .arg("--manifest-path=../../Cargo.toml")
    .arg("plugin")
    .arg("Directory Browser")
    .output()
    .expect("Unable to execute cargo run");

  let plugin_config = read_to_string("../platformer/addons/directory_browser/plugin.cfg")
    .expect("Unable to read gdnlib");
  let plugin_config_split = plugin_config.split("\n").collect::<Vec<&str>>();

  assert_eq!(plugin_config_split[0].trim(), "[plugin]");
  assert_eq!(
    plugin_config_split[1].trim(),
    "name = \"Directory Browser\""
  );
  assert_eq!(plugin_config_split[2].trim(), "description = \"\"");
  assert_eq!(plugin_config_split[3].trim(), "author = \"\"");
  assert_eq!(plugin_config_split[4].trim(), "version = \"1.0\"");
  assert_eq!(
    plugin_config_split[5].trim(),
    "script = \"../../rust_modules/directory_browser.gdns\""
  );

  set_current_dir("../").expect("Unable to change to parent directory");

  cleanup_test_files();

  Ok(())
}

#[test]
fn plugin_has_correct_initial_module_file() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot-rust-cli")?;
  cmd.arg("new").arg("platformer_modules").arg("platformer");

  cmd.assert().success();

  set_current_dir("platformer_modules").expect("Unable to change to library directory");
  Command::new("cargo")
    .arg("run")
    .arg("--manifest-path=../../Cargo.toml")
    .arg("plugin")
    .arg("Directory Browser")
    .output()
    .expect("Unable to execute cargo run");

  let plugin_base_gdns =
    read_to_string("src/directory_browser.rs").expect("Unable to read mod file");
  let plugin_base_gdns_split = plugin_base_gdns.split("\n").collect::<Vec<&str>>();

  assert_eq!(
    plugin_base_gdns_split[0],
    "use gdnative::api::EditorPlugin;"
  );
  assert_eq!(
    plugin_base_gdns_split[1],
    "use gdnative::nativescript::user_data;"
  );
  assert_eq!(plugin_base_gdns_split[2], "");
  assert_eq!(
    plugin_base_gdns_split[3],
    "#[derive(gdnative::NativeClass)]"
  );
  assert_eq!(plugin_base_gdns_split[4], "#[inherit(EditorPlugin)]");
  assert_eq!(
    plugin_base_gdns_split[5],
    "#[user_data(user_data::LocalCellData<DirectoryBrowser>)]"
  );
  assert_eq!(plugin_base_gdns_split[6], "pub struct DirectoryBrowser;");
  assert_eq!(plugin_base_gdns_split[7], "");
  assert_eq!(plugin_base_gdns_split[8], "#[gdnative::methods]");
  assert_eq!(plugin_base_gdns_split[9], "impl DirectoryBrowser {");
  assert_eq!(
    plugin_base_gdns_split[10].trim(),
    "fn new(_owner: &EditorPlugin) -> Self {"
  );
  assert_eq!(plugin_base_gdns_split[11].trim(), "DirectoryBrowser");
  assert_eq!(plugin_base_gdns_split[12].trim(), "}");
  assert_eq!(plugin_base_gdns_split[13], "");
  assert_eq!(plugin_base_gdns_split[14].trim(), "#[export]");
  assert_eq!(
    plugin_base_gdns_split[15].trim(),
    "fn _ready(&self, _owner: &EditorPlugin) {"
  );
  assert_eq!(
    plugin_base_gdns_split[16].trim(),
    "gdnative::godot_print!(\"hello, world.\");"
  );
  assert_eq!(plugin_base_gdns_split[17].trim(), "}");
  assert_eq!(plugin_base_gdns_split[18], "}");
  assert_eq!(plugin_base_gdns_split[19], "");

  set_current_dir("../").expect("Unable to change to parent directory");

  cleanup_test_files();

  Ok(())
}

#[test]
fn plugin_has_correct_gdns_file_in_godot_project_rust_modules() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot-rust-cli")?;
  cmd.arg("new").arg("platformer_modules").arg("platformer");

  cmd.assert().success();

  set_current_dir("platformer_modules").expect("Unable to change to library directory");
  Command::new("cargo")
    .arg("run")
    .arg("--manifest-path=../../Cargo.toml")
    .arg("plugin")
    .arg("Directory Browser")
    .output()
    .expect("Unable to execute cargo run");

  let plugin_base_gdns = read_to_string("../platformer/rust_modules/directory_browser.gdns")
    .expect("Unable to read gdnlib");
  let plugin_base_gdns_split = plugin_base_gdns.split("\n").collect::<Vec<&str>>();

  assert_eq!(
    plugin_base_gdns_split[0].trim(),
    "[gd_resource type=\"NativeScript\" load_steps=2 format=2]"
  );
  assert_eq!(plugin_base_gdns_split[1].trim(), "");
  assert_eq!(
    plugin_base_gdns_split[2].trim(),
    "[ext_resource path=\"res://platformer_modules.gdnlib\" type=\"GDNativeLibrary\" id=1]"
  );
  assert_eq!(plugin_base_gdns_split[3].trim(), "");
  assert_eq!(plugin_base_gdns_split[4].trim(), "[resource]");
  assert_eq!(plugin_base_gdns_split[5].trim(), "");
  assert_eq!(
    plugin_base_gdns_split[6].trim(),
    "resource_name = \"DirectoryBrowser\""
  );
  assert_eq!(
    plugin_base_gdns_split[7].trim(),
    "class_name = \"DirectoryBrowser\""
  );
  assert_eq!(plugin_base_gdns_split[8], "library = ExtResource( 1 )");

  set_current_dir("../").expect("Unable to change to parent directory");

  cleanup_test_files();

  Ok(())
}

#[test]
fn plugin_mod_and_handle_added_to_lib_file() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot-rust-cli")?;
  cmd.arg("new").arg("platformer_modules").arg("platformer");

  cmd.assert().success();

  set_current_dir("platformer_modules").expect("Unable to change to library directory");
  Command::new("cargo")
    .arg("run")
    .arg("--manifest-path=../../Cargo.toml")
    .arg("plugin")
    .arg("Directory Browser")
    .output()
    .expect("Unable to execute cargo run");

  let plugin_lib_file = read_to_string("src/lib.rs").expect("Unable to read lib file");
  let plugin_lib_file_split = plugin_lib_file.split("\n").collect::<Vec<&str>>();

  assert_eq!(plugin_lib_file_split[0], "mod directory_browser;");
  assert_eq!(
    plugin_lib_file_split[4].trim(),
    "handle.add_tool_class::<directory_browser::DirectoryBrowser>();"
  );

  set_current_dir("../").expect("Unable to change to parent directory");

  cleanup_test_files();

  Ok(())
}

#[test]
fn plugin_destroy_remove_from_library_lib_file_and_godot_project_rust_modules_and_plugin_directory(
) -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot-rust-cli")?;
  cmd.arg("new").arg("platformer_modules").arg("platformer");

  cmd.assert().success();

  set_current_dir("platformer_modules").expect("Unable to change to library directory");
  Command::new("cargo")
    .arg("run")
    .arg("--manifest-path=../../Cargo.toml")
    .arg("plugin")
    .arg("Directory Browser")
    .output()
    .expect("Unable to execute cargo run");
  Command::new("cargo")
    .arg("run")
    .arg("--manifest-path=../../Cargo.toml")
    .arg("destroy")
    .arg("Directory Browser")
    .output()
    .expect("Unable to execute cargo run");

  let lib_file = read_to_string("src/lib.rs").expect("Unable to read lib file");
  let lib_file_split = lib_file.split("\n").collect::<Vec<&str>>();

  let config_file = read_to_string("project.toml").expect("Unable to read config file");
  let config_split = config_file.split("\n").collect::<Vec<&str>>();

  let plugin_file_path = Path::new("src/directory_browser.rs");
  let plugin_gdns_file = Path::new("../platformer/rust_modules/directory_browser.gdns");
  let plugin_godot_directory = Path::new("../platformer/addons/directory_browser");

  assert_eq!(lib_file_split[0], "use gdnative::prelude::*;");
  assert_eq!(lib_file_split[1], "");
  assert_eq!(lib_file_split[2], "fn init(handle: InitHandle) {}");
  assert_eq!(lib_file_split[3], "");
  assert_eq!(lib_file_split[4], "godot_init!(init);");
  assert_eq!(config_split[1], "modules = []");
  assert_eq!(plugin_file_path.exists(), false);
  assert_eq!(plugin_gdns_file.exists(), false);
  assert_eq!(plugin_godot_directory.exists(), false);

  set_current_dir("../").expect("Unable to change to parent directory");

  cleanup_test_files();

  Ok(())
}

#[test]
fn plugin_destroy_create_three_modules_and_two_plugins_remove_one_module_and_one_plugin(
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
    .arg("plugin")
    .arg("File Parser")
    .output()
    .expect("Unable to execute cargo run");
  Command::new("cargo")
    .arg("run")
    .arg("--manifest-path=../../Cargo.toml")
    .arg("plugin")
    .arg("Directory Browser")
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
    .arg("Directory Browser")
    .output()
    .expect("Unable to execute cargo run");

  let lib_file = read_to_string("src/lib.rs").expect("Unable to read lib file");
  let lib_file_split = lib_file.split("\n").collect::<Vec<&str>>();

  let config_file = read_to_string("project.toml").expect("Unable to read config file");
  let config_split = config_file.split("\n").collect::<Vec<&str>>();

  let player_mod_file_path = Path::new("src/player.rs");
  let main_scene_mod_file_path = Path::new("src/main_scene.rs");
  let ship_mod_file_path = Path::new("src/ship.rs");
  let file_parser_mod_file_path = Path::new("src/file_parser.rs");
  let directory_browser_mod_file_path = Path::new("src/directory_browser.rs");

  let player_gdns_file = Path::new("../platformer/rust_modules/player.gdns");
  let main_scene_gdns_file = Path::new("../platformer/rust_modules/main_scene.gdns");
  let ship_gdns_file = Path::new("../platformer/rust_modules/ship.gdns");
  let file_parser_gdns_file = Path::new("../platformer/rust_modules/file_parser.gdns");
  let directory_browser_gdns_file = Path::new("../platformer/rust_modules/directory_browser.gdns");

  let file_parser_godot_plugin_dir = Path::new("../platformer/addons/file_parser");
  let directory_browser_godot_plugin_dir = Path::new("../platformer/addons/directory_browser");

  assert_eq!(lib_file_split[0], "mod file_parser;");
  assert_eq!(lib_file_split[1], "mod main_scene;");
  assert_eq!(lib_file_split[2], "mod player;");
  assert_eq!(lib_file_split[3], "use gdnative::prelude::*;");
  assert_eq!(lib_file_split[4], "");
  assert_eq!(lib_file_split[5], "fn init(handle: InitHandle) {");
  assert_eq!(
    lib_file_split[6].trim(),
    "handle.add_class::<player::Player>();"
  );
  assert_eq!(
    lib_file_split[7].trim(),
    "handle.add_tool_class::<file_parser::FileParser>();"
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
    "modules = [\"Player\", \"MainScene\", \"FileParser\"]"
  );

  assert_eq!(player_mod_file_path.exists(), true);
  assert_eq!(main_scene_mod_file_path.exists(), true);
  assert_eq!(file_parser_mod_file_path.exists(), true);
  assert_eq!(directory_browser_mod_file_path.exists(), false);
  assert_eq!(ship_mod_file_path.exists(), false);

  assert_eq!(player_gdns_file.exists(), true);
  assert_eq!(main_scene_gdns_file.exists(), true);
  assert_eq!(file_parser_gdns_file.exists(), true);
  assert_eq!(directory_browser_gdns_file.exists(), false);
  assert_eq!(ship_gdns_file.exists(), false);

  assert_eq!(file_parser_godot_plugin_dir.exists(), true);
  assert_eq!(directory_browser_godot_plugin_dir.exists(), false);

  set_current_dir("../").expect("Unable to change to parent directory");

  cleanup_test_files();

  Ok(())
}
