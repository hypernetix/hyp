/// E1404: Narrowing conversions (as)
/// Severity: MED
/// LLM confusion: 3 (MED)
///
/// Description: Converting from a larger number type to a smaller one can lose data. For example,
/// converting a 64-bit number to 32-bit truncates (cuts off) values that don't fit, silently losing
/// the upper bits. It's like trying to fit a 10-digit number into a 5-digit display - the extra
/// digits just disappear. Fix by using try_into() which returns an error if data would be lost,
/// or validate the range before converting.
///
/// Mitigation: Use `#![warn(clippy::cast_possible_truncation)]` to catch narrowing casts. Use
/// `try_into()` or `try_from()` for checked conversions. Validate that values are in range
/// before casting with `as`.

pub fn e1404_narrowing_conversion(x: i64) -> i32 {
    // PROBLEM E1404: Can lose data with 'as' cast
    x as i32
}

pub fn e1404_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
