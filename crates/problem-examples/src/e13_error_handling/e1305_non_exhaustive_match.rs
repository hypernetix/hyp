/// E1305: Non-exhaustive match on Result/Option
/// Severity: HIGH
/// LLM confusion: 2 (LOW)
///
/// Description: This code uses a wildcard pattern (`_`) to catch all error cases, which means
/// errors are not properly examined or handled. Different errors might need different responses
/// (like retrying vs. giving up), but this treats them all the same. It's like having a try/catch
/// that just returns a default value for any error without checking what went wrong. Fix by
/// explicitly matching on error types or at least logging the error before providing a default.
///
/// Mitigation: Avoid using `_` to catch Result/Option error cases. Use explicit pattern matching
/// or combinators like `unwrap_or_else()` that give you access to the error value. Log errors
/// before converting them to default values.

#[allow(clippy::manual_unwrap_or_default)]
pub fn e1305_non_exhaustive_match(result: Result<i32, String>) -> i32 {
    // PROBLEM E1305: Not handling all cases
    match result {
        Ok(v) => v,
        _ => 0, // Error case not properly handled
    }
}

pub fn e1305_entry() -> Result<(), Box<dyn std::error::Error>> {
    let result: Result<i32, String> = Ok(42);
    let _ = e1305_non_exhaustive_match(result);
    Ok(())
}
