/// E1306: Swallowing errors without logging
/// Severity: MED
/// LLM confusion: 3 (MED)
///
/// Description: This code converts errors to None (using `.ok()`) without logging what went
/// wrong. When something fails, there's no record of it anywhere - no log file, no error message,
/// nothing. You'll just see None and have no idea why the operation failed. It's like a function
/// that returns null on error but doesn't tell you what error occurred. Fix by logging errors
/// before converting them to Option, or return Result instead to preserve error information.
///
/// Mitigation: Use a logging framework and log errors before converting to Option. Consider
/// whether Option is the right return type - Result preserves error information. Use
/// `inspect_err()` to log errors while still converting to Option if needed.

pub fn e1306_swallow_errors() -> Option<i32> {
    let result = std::fs::read_to_string("file.txt");
    // PROBLEM E1306: Error converted to None without logging
    result.ok().and_then(|s| s.parse().ok())
}

pub fn e1306_entry() -> Result<(), Box<dyn std::error::Error>> {
    let _ = e1306_swallow_errors();
    Ok(())
}
