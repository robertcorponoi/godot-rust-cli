use regex::Regex;
use std::path::PathBuf;
use std::{fs::write, io::Result, path::Path, process::Command};

/// Writes Rust code to a file and then runs `rustfmt` to format it.
///
/// `path` - The path to the file to write the code to.
/// `code` - The Rust code to write to the file.
pub fn write_and_fmt<P: AsRef<Path>, S: ToString>(path: P, code: S) -> Result<()> {
    write(&path, code.to_string())?;

    Command::new("rustfmt").arg(path.as_ref()).spawn()?.wait()?;

    Ok(())
}

/// Takes a regex pattern to find matching lines and then another regex pattern
/// to find the module name within that match to find where the next module
/// definition can be placed.
///
/// # Arguments
///
/// `modules` - A copy of the current modules from the godot-rust-cli.toml file.
/// `lib_file_contents` - The string contents of the lib file to add the module to.
/// `module_name` - The name of the new module to add to the lib file.
pub fn get_insert_location(
    modules: &Vec<String>,
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

        // // If this capture group matches one of the modules from the godot-rust-cli.toml
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

/// Copies a file at the specified path to the destination path.
///
/// `path_to_file_to_copy` - The path to the file to copy.
/// `destination_path` - The destination path.
pub fn copy_file_to_location(path_to_file_to_copy: &PathBuf, destination_path: &PathBuf) {
    Command::new("cp")
        .arg(path_to_file_to_copy)
        .arg(destination_path)
        .output()
        .expect("Unable to copy file");
}
