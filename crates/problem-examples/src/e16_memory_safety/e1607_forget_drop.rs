/// E1607: Forgetting to drop
/// Severity: MED
/// LLM confusion: 3 (MED)
///
/// Description: `mem::forget` prevents a value's destructor from running, which means its
/// resources are never cleaned up. For a Vec, this leaks the heap-allocated memory. This is
/// safe (won't corrupt memory) but wastes resources. Fix by letting values drop normally, or
/// use `ManuallyDrop` if you truly need to prevent cleanup.
///
/// Mitigation: Avoid `mem::forget` unless you have a very specific reason. Use `ManuallyDrop`
/// for explicit control over when destructors run. Understand that forgetting is safe but can
/// leak resources. Use `#![warn(clippy::mem_forget)]` to detect forget calls.

pub fn e1607_forget_drop() {
    let data = vec![1, 2, 3];
    // PROBLEM E1607: Explicitly forgetting to drop, leaks memory
    std::mem::forget(data);
}

pub fn e1607_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
