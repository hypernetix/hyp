/// E1003: Direct use of unsafe code
/// Severity: HIGH
/// LLM confusion: 4 (HIGH)
///
/// Description: Unsafe code bypasses Rust's compile-time safety guarantees, allowing direct memory
/// manipulation and other operations that can lead to undefined behavior, crashes, or memory
/// corruption if used incorrectly. In a production environment, such issues can be critical.
/// While `unsafe` is sometimes necessary for performance or FFI, its use must be rigorously
/// justified and carefully audited. Every `unsafe` block requires a `// SAFETY:` comment
/// explaining precisely why the code is safe, detailing all invariants maintained and
/// assumptions made. Without this documentation, verifying correctness and safely maintaining
/// the code becomes extremely difficult, increasing the risk of severe bugs.
///
/// Mitigation: Try to avoid unsafe code in release builds or move it to isolated modules
/// to make it easier to audit.

pub fn e1003_unsafe_code() {
    let x = 42;
    let ptr = &x as *const i32;

    // PROBLEM E1003: Direct use of usafe code
    unsafe {
        // PROBLEM E1004: No safety comment explaining why this is safe
        let _value = *ptr;
    }
}

pub fn e1003_entry() -> Result<(), Box<dyn std::error::Error>> {
    e1003_unsafe_code();
    Ok(())
}
