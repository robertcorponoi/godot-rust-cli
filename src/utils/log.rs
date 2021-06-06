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

/// Logs the version of the program.
pub fn log_version() {
    let version_notice = format!(
        "{}{}",
        "godot-rust-cli".white().underline(),
        env!("CARGO_PKG_VERSION").white().underline()
    );
    log_styled_message_to_console(&version_notice, ConsoleColors::WHITE);
}
