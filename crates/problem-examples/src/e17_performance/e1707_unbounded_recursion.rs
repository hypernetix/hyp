/// E1707: Unbounded recursion
/// Severity: HIGH
/// LLM confusion: 3 (MED)
///
/// Description: Unbounded recursion is recursion without a proper base case for all inputs. This
/// function has a base case for n=0, but if called with a negative number, it recurses forever
/// (n-1 makes negative numbers more negative). This will overflow the stack and crash. Fix by
/// adding proper base cases for all possible inputs.
///
/// Mitigation: Ensure all recursive functions have base cases covering all inputs. Use unsigned
/// types (u32) when values should never be negative. Add assertions or validation at function
/// entry. Convert to iteration when recursion depth is unbounded.

pub fn e1707_unbounded_recursion(n: i32) -> i32 {
    // PROBLEM E1707: No base case for negative numbers
    if n == 0 {
        0
    } else {
        1 + e1707_unbounded_recursion(n - 1)
    }
}

pub fn e1707_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
