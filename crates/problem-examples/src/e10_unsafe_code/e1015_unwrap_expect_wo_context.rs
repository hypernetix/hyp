/// E1015: Unwrap/expect without context
/// Severity: HIGH
/// LLM confusion: 3 (MED)
///
/// Description: Using unwrap() or expect() crashes the program if the value is None or Err, but
/// provides no context about what went wrong or where. It's like catching an exception and just
/// printing "error" with no details - you know something failed but not what or why. This makes
/// debugging very difficult. Fix by using proper error handling with Result/Option, or at minimum
/// use expect() with a descriptive message explaining what was expected.
///
/// Mitigation: Use `#![warn(clippy::unwrap_used)]` to catch unwrap calls. Prefer pattern matching,
/// `if let`, or the `?` operator for error handling. If unwrap is necessary, add a comment
/// explaining why it's safe. Use `expect("descriptive message")` instead of bare `unwrap()`.

pub fn e1015_unwrap_expect_wo_context() {
    let data = Some(42);

    // PROBLEM E1002: direct unwrap/expect
    // PROBLEM E1015: unwrap without context
    let _value = data.unwrap();

    // PROBLEM E1002: direct unwrap/expect
    // PROBLEM E1015: expect with poor message
    let _value2 = data.expect("failed");
}

pub fn e1015_entry() -> Result<(), Box<dyn std::error::Error>> {
    e1015_unwrap_expect_wo_context();
    Ok(())
}
