/// E1304: Using unwrap() in error paths
/// Severity: HIGH
/// LLM confusion: 3 (MED)
///
/// Description: This function returns a Result type (indicating it can handle errors gracefully),
/// but internally uses `unwrap()` which crashes the program if an error occurs. This defeats the
/// entire purpose of returning Result. It's like a function that promises to handle errors but
/// actually just crashes when things go wrong. Fix by using the `?` operator to pass errors up
/// to the caller instead of crashing.
///
/// Mitigation: Use `#![warn(clippy::unwrap_in_result)]` to catch unwrap/expect in functions
/// returning Result. Use the `?` operator to propagate errors, or use `map_err()` to convert
/// error types when needed.

pub fn e1304_unwrap_in_error_path() -> Result<i32, String> {
    let data = std::fs::read_to_string("data.txt").map_err(|e| e.to_string())?;

    // PROBLEM E1304: unwrap() in a function that returns Result
    // PROBLEM E1002: direct unwrap/expect
    let num: i32 = data.trim().parse().unwrap();
    Ok(num)
}

pub fn e1304_entry() -> Result<(), Box<dyn std::error::Error>> {
    let _ = e1304_unwrap_in_error_path();
    Ok(())
}
