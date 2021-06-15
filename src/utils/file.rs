use std::path::PathBuf;
use std::{fs::write, io::Result, path::Path, process::Command};

/// Writes Rust code to a file and then runs `rustfmt` to format it.
///
/// # Arguments
/// 
/// `path` - The path to the file to write the code to.
/// `code` - The Rust code to write to the file.
pub fn write_and_fmt<P: AsRef<Path>, S: ToString>(path: P, code: S) -> Result<()> {
    write(&path, code.to_string())?;

    Command::new("rustfmt").arg(path.as_ref()).spawn()?.wait()?;

    Ok(())
}

/// Copies a file at the specified path to the destination path.
///
/// # Arguments
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