/// E1401: Integer overflow/underflow
/// Severity: HIGH
/// LLM confusion: 3 (MED)
///
/// Description: Integer overflow happens when arithmetic produces a number too large for the
/// variable type. In release builds, the number wraps around (like an odometer going past its max),
/// causing subtle bugs. For example, adding 1 to the maximum value gives you the minimum value.
/// It's like a calculator that shows 0 when you go past 999. Fix by using checked arithmetic
/// methods like checked_add() or use larger integer types when overflow is possible.
///
/// Mitigation: Use `#![warn(clippy::integer_arithmetic)]` to catch unchecked arithmetic. Use
/// `checked_add()`, `saturating_add()`, or `wrapping_add()` to make overflow behavior explicit.
/// Enable overflow checks in release builds with `overflow-checks = true` in Cargo.toml.

pub fn e1401_integer_overflow(x: u8) -> u8 {
    // PROBLEM E1401: Can overflow in release mode
    x + 100
}

pub fn e1401_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
