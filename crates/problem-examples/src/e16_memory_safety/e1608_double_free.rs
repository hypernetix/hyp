/// E1608: Double free
/// Severity: HIGH
/// LLM confusion: 5 (HIGHEST)
///
/// Description: Double free occurs when you try to deallocate the same memory twice. ManuallyDrop
/// prevents automatic dropping, but calling `ManuallyDrop::drop()` twice manually drops the same
/// value twice. This causes undefined behavior - the second drop operates on freed memory. Fix by
/// ensuring each value is dropped at most once.
///
/// Mitigation: Be extremely careful with `ManuallyDrop` - it's easy to create double-free bugs.
/// Use `ManuallyDrop::take()` to extract the value, which prevents double-drop. Avoid manual
/// memory management when possible. Use Miri to detect double-free bugs.

pub fn e1608_double_free() {
    use std::mem::ManuallyDrop;

    let data = vec![1, 2, 3];
    let mut manual = ManuallyDrop::new(data);

    // PROBLEM E1003: Direct use of unsafe code
    unsafe {
        // PROBLEM E1004: No safety documentation
        // PROBLEM E1608: Dropping twice
        ManuallyDrop::drop(&mut manual);
        ManuallyDrop::drop(&mut manual);
    }
}

pub fn e1608_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
