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
/// ## The Wrapping Problem
///
/// ```text
/// let x: u8 = 255;
/// let y = x + 1;  // In debug: panic! In release: y = 0 (wrapped!)
///
/// let z: u8 = 200;
/// let w = z + 100; // In debug: panic! In release: w = 44 (wrapped!)
/// ```
///
/// ## Why This Matters
///
/// 1. **Silent corruption**: Release builds wrap silently, debug builds panic
/// 2. **Security vulnerabilities**: Buffer size calculations can wrap to small values
/// 3. **Financial errors**: Money calculations can produce wrong amounts
/// 4. **Logic bugs**: Loop counters can wrap causing infinite loops
///
/// ## The Right Solutions
///
/// ### Option 1: Use checked_* methods
/// ```rust
/// fn safe_add(x: u8, y: u8) -> Option<u8> {
///     x.checked_add(y)
/// }
/// ```
///
/// ### Option 2: Use saturating_* methods
/// ```rust
/// fn capped_add(x: u8, y: u8) -> u8 {
///     x.saturating_add(y)  // Returns 255 if it would overflow
/// }
/// ```
///
/// ### Option 3: Use wrapping_* when wrapping is intentional
/// ```rust
/// fn hash_combine(a: u64, b: u64) -> u64 {
///     a.wrapping_add(b).wrapping_mul(31)  // Intentional wrapping
/// }
/// ```
///
/// Mitigation: Use `#![warn(clippy::integer_arithmetic)]` to catch unchecked arithmetic. Use
/// `checked_add()`, `saturating_add()`, or `wrapping_add()` to make overflow behavior explicit.
/// Enable overflow checks in release builds with `overflow-checks = true` in Cargo.toml.

// ============================================================================
// DANGEROUS PATTERNS - NEVER DO THIS
// ============================================================================

/// PROBLEM E1401: Can overflow in release mode
pub fn e1401_bad_overflow(x: u8) -> u8 {
    // PROBLEM E1401: Can overflow in release mode
    x + 100
}

/// PROBLEM E1401: Multiplication overflow
pub fn e1401_bad_multiply(x: u32, y: u32) -> u32 {
    x * y // Can easily overflow
}

/// PROBLEM E1401: Underflow with subtraction
pub fn e1401_bad_underflow(x: u32, y: u32) -> u32 {
    x - y // Underflows if y > x
}

/// PROBLEM E1401: Overflow in loop counter
pub fn e1401_bad_loop_overflow() -> u8 {
    let mut sum: u8 = 0;
    for i in 0..200u8 {
        sum += i; // Will overflow!
    }
    sum
}

/// Entry point for problem demonstration
pub fn e1401_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

// ============================================================================
// GOOD ALTERNATIVES
// ============================================================================

/// GOOD: Use checked_add for safe addition
pub fn e1401_good_checked_add(x: u8, y: u8) -> Option<u8> {
    x.checked_add(y)
}

/// GOOD: Use saturating_add to cap at max value
pub fn e1401_good_saturating_add(x: u8, y: u8) -> u8 {
    x.saturating_add(y) // Returns 255 if would overflow
}

/// GOOD: Use wrapping_add when wrapping is intentional
pub fn e1401_good_wrapping_add(x: u8, y: u8) -> u8 {
    x.wrapping_add(y) // Explicit that wrapping is expected
}

/// GOOD: Use overflowing_add to detect overflow
pub fn e1401_good_overflowing_add(x: u8, y: u8) -> (u8, bool) {
    x.overflowing_add(y) // Returns (result, did_overflow)
}

/// GOOD: Use larger type when overflow is possible
pub fn e1401_good_wider_type(x: u8, y: u8) -> u16 {
    (x as u16) + (y as u16) // Cannot overflow
}

/// GOOD: Checked multiplication
pub fn e1401_good_checked_multiply(x: u32, y: u32) -> Option<u32> {
    x.checked_mul(y)
}

/// GOOD: Checked subtraction
pub fn e1401_good_checked_sub(x: u32, y: u32) -> Option<u32> {
    x.checked_sub(y)
}

/// GOOD: Safe loop with checked arithmetic
pub fn e1401_good_safe_loop() -> Option<u16> {
    let mut sum: u16 = 0;
    for i in 0..200u16 {
        sum = sum.checked_add(i)?;
    }
    Some(sum)
}

// ============================================================================
// GOOD EXAMPLES unit tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checked_add_success() {
        assert_eq!(e1401_good_checked_add(100, 50), Some(150));
    }

    #[test]
    fn test_checked_add_overflow() {
        assert_eq!(e1401_good_checked_add(200, 100), None);
    }

    #[test]
    fn test_saturating_add_caps() {
        assert_eq!(e1401_good_saturating_add(200, 100), 255);
    }

    #[test]
    fn test_wrapping_add_wraps() {
        assert_eq!(e1401_good_wrapping_add(200, 100), 44); // 300 % 256 = 44
    }

    #[test]
    fn test_overflowing_add_detects() {
        assert_eq!(e1401_good_overflowing_add(200, 100), (44, true));
        assert_eq!(e1401_good_overflowing_add(100, 50), (150, false));
    }

    #[test]
    fn test_wider_type_no_overflow() {
        assert_eq!(e1401_good_wider_type(200, 200), 400);
    }

    #[test]
    fn test_checked_sub_underflow() {
        assert_eq!(e1401_good_checked_sub(5, 10), None);
        assert_eq!(e1401_good_checked_sub(10, 5), Some(5));
    }

    #[test]
    fn test_safe_loop() {
        let result = e1401_good_safe_loop();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 19900); // Sum of 0..200
    }
}
