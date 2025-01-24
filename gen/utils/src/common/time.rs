use chrono::{DateTime, Local};

/// Get the current local time.
pub fn local_time() -> DateTime<Local> {
    Local::now()
}

/// Get the current local time in ISO8601 format.
pub fn local_time_iso8601() -> String {
    let now = Local::now();
    now.to_rfc3339()
}

/// Get the current local time in the default format.
/// The default format is `%Y-%m-%d %H:%M:%S`.
pub fn local_time_default() -> String {
    let now = Local::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Get the current local time in the specified format.
pub fn local_time_format(format: &str) -> String {
    let now = Local::now();
    now.format(format).to_string()
}

#[cfg(test)]
mod test_format_time{
    #[test]
    fn test_local_time() {
        let time1 = super::local_time();
        let time2 = super::local_time_iso8601();
        let time3 = super::local_time_default();
        let time4 = super::local_time_format("%Y-%m-%d %H:%M:%S");
        println!("local time: {:?}", time1);
        println!("local time iso8601: {:?}", time2);
        println!("local time default: {:?}", time3);
        println!("local time format: {:?}", time4);
    }
}