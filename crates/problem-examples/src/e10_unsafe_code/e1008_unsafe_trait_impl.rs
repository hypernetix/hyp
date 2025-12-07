/// E1008: Unsafe trait implementation
/// Severity: HIGH
/// LLM confusion: 4 (HIGH)
///
/// Description: Implementing an unsafe trait (like Send or Sync) promises the compiler that your
/// type meets certain safety requirements. If you implement these traits incorrectly, you can cause
/// data races, memory corruption, or other undefined behavior - and the compiler can't check if
/// you're right. It's like promising "this type is thread-safe" when it actually isn't. The compiler
/// trusts your promise and allows multi-threaded access, which then causes bugs.
///
/// Mitigation: Only implement unsafe traits when you fully understand their safety requirements.
/// Document why your implementation is safe. Use `#![warn(clippy::undocumented_unsafe_blocks)]`.
/// Prefer using safe wrappers (Arc, Mutex) instead of implementing Send/Sync manually. Test
/// thoroughly with thread sanitizers.

struct MyType {
    _data: *const i32,
}

// PROBLEM E1008: Implementing unsafe trait without proper safety guarantees
unsafe impl Send for MyType {}

pub fn e1008_entry() -> Result<(), Box<dyn std::error::Error>> {
    let _ = MyType {
        _data: std::ptr::null(),
    };
    Ok(())
}
