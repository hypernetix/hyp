/// E1705: Clone in hot path
/// Severity: LOW
/// LLM confusion: 2 (LOW)
///
/// Description: Cloning large data structures in performance-critical code (hot paths) is
/// expensive. This code clones a Vec 1000 times in a loop, copying all the data each time. Fix
/// by using references instead of clones, or restructure the algorithm to avoid repeated cloning.
///
/// Mitigation: Use `#![warn(clippy::clone_on_ref_ptr)]` to catch suspicious clones. Profile to
/// identify expensive clones. Use references or `Cow` (Clone on Write) to avoid clones. Consider
/// if you really need owned data or if borrowing would work.

#[allow(clippy::ptr_arg)]
pub fn e1705_clone_in_hot_path(data: &Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for _ in 0..1000 {
        // PROBLEM E1705: Cloning large data structure repeatedly
        let copy = data.clone();
        result.extend(copy);
    }
    result
}

pub fn e1705_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
