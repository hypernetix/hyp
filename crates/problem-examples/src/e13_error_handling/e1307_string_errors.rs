/// E1307: Using String for error types
/// Severity: MED
/// LLM confusion: 2 (LOW)
///
/// Description: Using plain strings as error types loses important information about what kind of
/// error occurred. Strings can't be pattern-matched or handled differently based on error type,
/// and they don't follow standard error conventions. It's like throwing exceptions with just a
/// message string instead of specific exception classes - you can't catch specific errors to
/// handle them differently. Fix by creating proper error enums or using existing error types.
///
/// Mitigation: Use `#![warn(clippy::string_slice_as_bytes)]` and create custom error enums or
/// use the `thiserror` crate to derive Error implementations. String errors should only be used
/// in quick prototypes, never in production code.

pub fn e1307_string_errors() -> Result<i32, String> {
    // PROBLEM E1307: String errors lose type information
    Err("Something went wrong".to_string())
}

pub fn e1307_entry() -> Result<(), Box<dyn std::error::Error>> {
    let _ = e1307_string_errors();
    Ok(())
}
