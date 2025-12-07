/// E1706: Non-tail recursion
/// Severity: MED
/// LLM confusion: 3 (MED)
///
/// Description: Tail recursion is when a function's last operation is calling itself. Some
/// languages optimize this to avoid stack growth, but Rust doesn't guarantee tail call
/// optimization. This recursive factorial is NOT tail-recursive because it multiplies after
/// the recursive call. Fix by converting to iteration or using an accumulator for tail recursion.
///
/// Mitigation: Convert recursive functions to iterative loops when possible. Use an accumulator
/// parameter to make recursion tail-recursive. Be aware that Rust doesn't guarantee TCO (tail
/// call optimization). For deep recursion, use iteration or trampolining.

pub fn e1706_non_tail_recursion(n: u64) -> u64 {
    // PROBLEM E1706: Not tail-recursive, can overflow stack
    if n == 0 {
        1
    } else {
        n * e1706_non_tail_recursion(n - 1)
    }
}

pub fn e1706_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
