use dunce::canonicalize;
use std::{
  env::current_dir,
  fs::{read_to_string, write},
  io::Result,
  path::{Path, PathBuf},
  process::{exit, Command},
};

use crate::config::Configuration;
use colored::Colorize;
use regex::Regex;

/// Defines the colors that can be passed to `print_to_console` as a second
/// argument to change the color of the printed text.
pub enum ConsoleColors {
  RED,
  WHITE,
  GREEN,
  CYAN,
}

/// Prints a message to the console. The second parameter can be provided as
/// the color to print with a default color of white.
pub fn print_to_console(message: &str, color: ConsoleColors) {
  match color {
    ConsoleColors::RED => println!("{}", message.red()),
    ConsoleColors::GREEN => println!("{}", message.green()),
    ConsoleColors::CYAN => println!("{}", message.cyan()),
    _ => println!("{}", message.white()),
  }
}

/// Returns the absolute path from a PathBuf.
///
/// # Arguments
///
/// `buf` - The PathBuf to return the absolute path of.
pub fn get_absolute_path(buf: PathBuf) -> PathBuf {
  return if !buf.is_absolute() {
    absolute_path(buf)
      .expect("Unable to create absolute path")
      .as_path()
      .to_owned()
  } else {
    Path::new(&buf).to_path_buf()
  };
}

/// Writes Rust code to a file and then runs `rustfmt` to format it.
///
/// `path` - The path to the file to write the code to.
/// `code` - The Rust code to write to the file.
pub fn write_and_fmt<P: AsRef<Path>, S: ToString>(path: P, code: S) -> Result<()> {
  write(&path, code.to_string())?;

  Command::new("rustfmt").arg(path.as_ref()).spawn()?.wait()?;

  Ok(())
}

/// Checks to see whether a command is being used from the library's directory
/// or not. Nothing is returned but if we are not in the library directory then
/// we print an error to the console and return early.
pub fn check_if_lib_dir() {
  if !get_project_toml_path().exists() {
    print_to_console(
      "This command must be used from the library directory",
      ConsoleColors::RED,
    );
    exit(1);
  }
}

/// Returns the contents of the project.toml as a Config object.
pub fn get_project_toml_as_object() -> Configuration {
  let project_toml_path = get_project_toml_path();
  let project_toml_string = read_to_string(project_toml_path).expect("Unable to read project.toml");
  return toml::from_str(&project_toml_string).expect("Unable to parse project.toml");
}

/// Returns the path to the dynamic libraries.
pub fn get_dynamic_libraries_path() -> PathBuf {
  let curr_dir = current_dir().expect("Unable to get current directory");
  return Path::new(&curr_dir).join("target").join("debug");
}

/// Writes the new contents to the project.toml.
///
/// `new_project_toml_contents` - The new contents of the project.toml to write to the file.
pub fn set_project_toml_contents(new_project_toml_contents: Configuration) {
  let project_toml_path = get_project_toml_path();
  let new_project_toml_string = toml::to_string(&new_project_toml_contents)
    .expect("Unable to convert godot-rust-cli.toml to string");

  match write(project_toml_path, new_project_toml_string) {
    Ok(_) => (),
    Err(e) => {
      print_to_console(&e.to_string(), ConsoleColors::RED);
      exit(1);
    }
  }
}

/// Returns the path to the project.toml.
fn get_project_toml_path() -> PathBuf {
  let curr_dir = current_dir().expect("Unable to get current directory");
  return Path::new(&curr_dir).join("project.toml");
}

/// Returns the build file extension for windows.
#[cfg(target_os = "windows")]
pub fn get_dynamic_library_ext() -> &'static str {
  return "dll";
}

/// Returns the build file extension for linux.
#[cfg(target_os = "linux")]
pub fn get_dynamic_library_ext() -> &'static str {
  return "so";
}

/// Returns the build file extension for macos.
#[cfg(target_os = "macos")]
pub fn get_dynamic_library_ext() -> &'static str {
  return "dylib";
}

/// Indicates whether a module is present in the project.toml or not.
///
/// # Arguments
///
/// `modules` - The modules from the project.toml file.
/// `module_name` - The module to check if exists or not.
pub fn is_module_in_project_toml(modules: &Vec<String>, module_name: &str) -> bool {
  if modules.iter().any(|i| i == module_name) {
    true
  } else {
    false
  }
}

/// Gets the name of the library from the current directory path.
pub fn get_library_name() -> String {
  let current_dir = std::env::current_dir().expect("Unable to get current directory");
  let lib_name = current_dir.file_name().expect("Unable to get library name");
  let lib_name_str = lib_name
    .to_str()
    .expect("Unable to convert library name to str");

  return lib_name_str.to_string();
}

/// Returns the absolute path from a relative path.
///
/// # Arguments
///
/// `path` - The relative path to get the absolute path of.
fn absolute_path<P>(path: P) -> Result<PathBuf>
where
  P: AsRef<Path>,
{
  let path = path.as_ref();
  let mut absolute_path = if path.is_absolute() {
    path.to_path_buf()
  } else {
    current_dir()?.join(path)
  };

  match canonicalize(&absolute_path) {
    Ok(v) => absolute_path = v,
    Err(_e) => {
      let parent = &absolute_path
        .parent()
        .expect("Unable to get the absolute path's parent");
      let basename = &absolute_path
        .file_stem()
        .expect("Unable to get the absolute path's basename");
      let parent_canon = canonicalize(parent).expect("Unable to canonicalize parent directory");

      absolute_path = parent_canon.join(basename);
    }
  }

  Ok(absolute_path)
}

/// Takes a regex pattern to find matching lines and then another regex pattern
/// to find the module name within that match to find where the next module
/// definition can be placed.
///
/// # Arguments
///
/// `modules` - A copy of the current modules from the project.toml file.
/// `lib_file_contents` - The string contents of the lib file to add the module to.
/// `module_name` - The name of the new module to add to the lib file.
pub fn get_insert_location(
  modules: Vec<String>,
  lib_file_contents: &String,
  line_pattern: &str,
  // module_name_pattern: &str,
  is_first: bool,
) -> (usize, Vec<String>) {
  // Since we want to add our new module after all of the currently existing
  // ones, we need to keep updating the insert position to the end position of
  // the last module found.
  let mut insert_pos = 0;

  // Create a copy of the modules vector so that we don't have to modify the
  // original one.
  let modules_copy = modules.clone();

  // Create the regex pattern used to check for the lines that might contain
  // our module definitions.
  let line_regex = Regex::new(line_pattern).expect("Unable to create regex");

  for line_match in line_regex.find_iter(&lib_file_contents) {
    if is_first {
      // If this is the first module to be added, meaning there's only 1 entry in
      // the modules vec, then we don't have to search for other modules and can
      // just write it within the init function.
      return (line_match.end(), modules_copy);
    }

    // // Otherwise, we need to check for existing module declerations.
    // let line_str = line_match.as_str();

    // // Create the regex pattern used to find the name of the module within the
    // // line.
    // let module_name_pattern = Regex::new(module_name_pattern).expect("Unable to create regex");
    // let module_name_match = module_name_pattern
    //   .captures(&line_str)
    //   .expect("Unable to get captures");
    // This regex returns two captures groups and we always want the second
    // group because it doesn't contain any symbols.
    // let module_name = module_name_match
    //   .get(1)
    //   .expect("Unable to get capture group");

    // // If this capture group matches one of the modules from the project.toml
    // // then we remove that module since it already exists.
    // let module_to_remove_index = modules_copy
    //   .iter()
    //   .position(|x| *&x.to_lowercase() == module_name.as_str())
    //   .expect("Unable get index of module to remove");
    // modules_copy.remove(module_to_remove_index);

    // New content will always go at the end of the current content so we keep
    // setting the insert position to the end position of the current match.
    insert_pos = line_match.end();
  }

  (insert_pos, modules_copy)
}
