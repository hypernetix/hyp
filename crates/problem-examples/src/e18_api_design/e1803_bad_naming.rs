/// E1803: Bad naming
/// Severity: LOW
/// LLM confusion: 1 (LOW)
///
/// Description: Function names should clearly describe what the function does. This function
/// multiplies two numbers and adds 42, but the name gives no indication of this behavior. Poor
/// naming makes code hard to understand and maintain. Fix by using descriptive names that explain
/// the function's purpose.
///
/// Mitigation: Use descriptive, verb-based names for functions. Follow Rust naming conventions:
/// snake_case for functions, describe what the function does. Use `#![warn(clippy::module_name_repetitions)]`
/// to catch redundant naming. Good names are documentation.

pub fn e1803_bad_naming(x: i32, y: i32) -> i32 {
    // PROBLEM E1803: Function name doesn't describe what it does
    x * y + 42
}

pub fn e1803_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
