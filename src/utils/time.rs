use chrono::offset::Local;

/// Returns the current time formatted as `YYYY-MM-DD HH:MM:SS`.
pub fn get_current_datetime_formatted() -> String {
    let dt = Local::now();
    return dt.format("%Y-%m-%d %H:%M:%S").to_string();
}
