use colored::Colorize;

/// Logs an error message to the console.
///
/// # Arguments
///
/// `message` - The message to log to the console.
pub fn log_error_to_console(message: &str) {
    println!("Error: {}", message.red());
}

/// Logs an info message to the console.
///
/// # Arguments
///
/// `message` - The message to log to the console.
pub fn log_info_to_console(message: &str) {
    println!("Info: {}", message.cyan());
}

/// Logs a success message to the console.
///
/// # Arguments
///
/// `message` - The message to log to the console.
pub fn log_success_to_console(message: &str) {
    println!("Success: {}", message.green());
}
