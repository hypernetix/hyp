/// E1407: Lossy float to int conversion
/// Severity: MED
/// LLM confusion: 3 (MED)
///
/// Description: Converting floating-point numbers to integers with 'as' truncates the decimal part
/// and can overflow if the float is too large. For example, converting a very large float to a
/// 32-bit integer produces undefined behavior. It's like rounding a decimal to a whole number but
/// also risking overflow if the number is too big. Fix by checking that floats are in valid range
/// before converting, or use explicit rounding functions.
///
/// Mitigation: Use `#![warn(clippy::cast_possible_truncation)]` and
/// `#![warn(clippy::float_to_int_without_bounds)]`. Check that floats are in valid range before
/// converting. Consider using `round()`, `floor()`, or `ceil()` to make rounding explicit.

pub fn e1407_lossy_float_conversion(x: f64) -> i32 {
    // PROBLEM E1407: Can lose fractional part and overflow
    x as i32
}

pub fn e1407_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
