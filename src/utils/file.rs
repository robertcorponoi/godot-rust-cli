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
