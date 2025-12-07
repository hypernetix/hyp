/// E1601: Aliasing violation
/// Severity: HIGH
/// LLM confusion: 5 (HIGHEST)
///
/// Description: Aliasing violations occur when you have multiple mutable pointers to the same
/// memory location, which breaks Rust's safety guarantees. Modifying memory through one pointer
/// while another exists can cause undefined behavior because the compiler assumes mutable
/// references are exclusive. Fix by ensuring only one mutable reference exists at a time, or
/// use proper synchronization.
///
/// Mitigation: Avoid creating multiple mutable raw pointers to the same data. Use safe Rust
/// borrowing rules instead of raw pointers. If raw pointers are necessary, carefully document
/// aliasing assumptions and ensure they don't overlap in time. Consider using `UnsafeCell` for
/// interior mutability.

pub fn e1601_aliasing_violation() {
    let mut data = vec![1, 2, 3];
    let ptr1 = data.as_mut_ptr();
    let ptr2 = data.as_mut_ptr();

    // PROBLEM E1003: Direct use of unsafe code
    unsafe {
        // PROBLEM E1004: No safety documentation
        // PROBLEM E1601: Multiple mutable aliases to same memory
        *ptr1 = 10;
        *ptr2 = 20;
    }
}

pub fn e1601_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
