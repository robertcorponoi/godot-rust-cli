use colored::Colorize;

/// Defines the colors that can be passed to commands that log to the console
/// to change the color of the logged text.
pub enum ConsoleColors {
    RED,
    WHITE,
    GREEN,
    CYAN,
}

/// Logs a message to the console with the provided `ConsoleColor`.
///
/// # Arguments
///
/// `message` - The message to log to the console.
/// `color` - The `ConsoleColor` to use to style the message in the console.
pub fn log_styled_message_to_console(message: &str, color: ConsoleColors) {
    match color {
        ConsoleColors::RED => println!("{}", message.red()),
        ConsoleColors::GREEN => println!("{}", message.green()),
        ConsoleColors::CYAN => println!("{}", message.cyan()),
        _ => println!("{}", message.white()),
    }
}

/// Logs the version of the cli.
pub fn log_version() {
    let version_notice = format!(
        "{}{}",
        "godot-rust-cli".white().underline(),
        env!("CARGO_PKG_VERSION").white().underline()
    );
    log_info_to_console(&version_notice);
}

/// Logs an error message to the console.
///
/// # Arguments
///
/// `message` - The message to log to the console.
pub fn log_error_to_console(message: &str) {
    println!("Error: {:?}", message.red());
}

/// Logs an info message to the console.
///
/// # Arguments
///
/// `message` - The message to log to the console.
pub fn log_info_to_console(message: &str) {
    println!("Error: {:?}", message.cyan());
}

/// Logs a success message to the console.
///
/// # Arguments
///
/// `message` - The message to log to the console.
pub fn log_success_to_console(message: &str) {
    println!("Error: {:?}", message.green());
}
