/// E1406: Signed/unsigned mismatch
/// Severity: MED
/// LLM confusion: 3 (MED)
///
/// Description: Comparing signed numbers (can be negative) with unsigned numbers (always positive)
/// can give wrong results. Negative signed values become huge when treated as unsigned (like -1
/// becoming 4294967295). It's like comparing temperatures in Celsius with raw binary values - the
/// comparison doesn't make sense. Fix by ensuring both values have the same signedness (both signed
/// or both unsigned) before comparing.
///
/// Mitigation: Use `#![warn(clippy::cast_sign_loss)]` to catch signed-to-unsigned casts. Keep
/// values in the same signedness domain. Use explicit range checks instead of casting for
/// comparisons.

pub fn e1406_signed_unsigned_mismatch(signed: i32, unsigned: u32) -> bool {
    // PROBLEM E1406: Comparing signed and unsigned can be problematic
    signed as u32 > unsigned
}

pub fn e1406_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
