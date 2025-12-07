/// E1402: Division by zero
/// Severity: HIGH
/// LLM confusion: 2 (LOW)
///
/// Description: Dividing by zero crashes the program immediately. This code doesn't check if the
/// divisor is zero before dividing, which will cause a runtime crash with certain inputs. It's
/// like a calculator that crashes when you press divide by zero. Fix by checking for zero before
/// dividing, using checked_div() which returns None instead of crashing, or ensuring the divisor
/// is never zero through program logic.
///
/// Mitigation: Use `checked_div()` which returns `None` for division by zero instead of panicking.
/// Add assertions or validation to ensure divisors are non-zero. Use the `NonZeroI32` type when
/// you need to guarantee a value is never zero.

pub fn e1402_division_by_zero(x: i32, y: i32) -> i32 {
    // PROBLEM E1402: No check for zero divisor
    x / y
}

pub fn e1402_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
