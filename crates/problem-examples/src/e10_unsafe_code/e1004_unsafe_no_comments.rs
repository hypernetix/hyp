/// E1004: Unsafe without comments
/// Severity: HIGH
/// LLM confusion: 4 (HIGH)
///
/// Description: This code uses unsafe operations without documenting the safety requirements. Unsafe
/// code can cause undefined behavior (crashes, data corruption, security vulnerabilities) if the
/// safety requirements aren't met. Every unsafe block needs a SAFETY comment explaining what makes
/// it safe - what conditions must be true, what the caller must guarantee. Without documentation,
/// future maintainers can't verify the code is correct.
///
/// Mitigation: Add `// SAFETY:` comments before all unsafe blocks. Document preconditions,
/// invariants, and why the operation is safe. Use `#![forbid(unsafe_op_in_unsafe_fn)]` to require
/// explicit unsafe blocks even in unsafe functions. Minimize and isolate unsafe code.

#[allow(clippy::useless_vec)]
pub fn e1004_unsafe_no_comments() {
    let data = vec![1, 2, 3, 4, 5];
    let ptr = data.as_ptr();

    // PROBLEM E1003: Direct use of unsafe code
    unsafe {
        // PROBLEM E1004: No safety documentation
        let _first = *ptr;
    }
}

pub fn e1004_entry() -> Result<(), Box<dyn std::error::Error>> {
    e1004_unsafe_no_comments();
    Ok(())
}
