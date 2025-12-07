/// E1408: Unchecked array indexing
/// Severity: HIGH
/// LLM confusion: 2 (LOW)
///
/// Description: Accessing an array element without checking if the index is valid will crash the
/// program if the index is out of bounds. While this is memory-safe (won't corrupt memory), it will
/// panic and crash. It's like trying to access the 100th element of a 10-element array - it crashes
/// instead of returning garbage. Fix by using .get() which returns None for invalid indices, or
/// validate indices before use.
///
/// Mitigation: Use `.get(idx)` which returns `Option<&T>` instead of panicking. Validate indices
/// before use. Use iterators instead of manual indexing when possible. Consider using
/// `get_unchecked()` only in performance-critical code with proven bounds.

pub fn e1408_unchecked_indexing(data: &[i32], idx: usize) -> i32 {
    // PROBLEM E1408: Can panic if idx is out of bounds
    data[idx]
}

pub fn e1408_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
