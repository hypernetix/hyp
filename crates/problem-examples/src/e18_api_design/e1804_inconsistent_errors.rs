/// E1804: Inconsistent error types
/// Severity: MED
/// LLM confusion: 2 (LOW)
///
/// Description: Using different error types for similar functions makes error handling inconsistent
/// and difficult. One function returns `io::Error`, another returns `String`. Callers can't handle
/// errors uniformly. Fix by using consistent error types across your API - define a custom error
/// type or use a common error type.
///
/// Mitigation: Define a custom error enum for your crate using `thiserror`. Use the same error
/// type for all functions in a module or crate. Use `anyhow` for applications where you just need
/// to propagate errors. Consistent error types make APIs easier to use.

pub fn e1804_inconsistent_errors_1() -> Result<i32, std::io::Error> {
    Ok(42)
}

pub fn e1804_inconsistent_errors_2() -> Result<i32, String> {
    // PROBLEM E1804: Different error types in similar functions
    Ok(42)
}

pub fn e1804_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
