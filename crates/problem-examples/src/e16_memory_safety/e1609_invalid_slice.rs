/// E1609: Invalid slice creation
/// Severity: HIGH
/// LLM confusion: 4 (HIGH)
///
/// Description: Creating a slice from raw parts requires the pointer and length to be valid.
/// This code creates a slice claiming to have 100 elements when the actual data only has 3.
/// Accessing elements beyond the actual data causes undefined behavior. Fix by ensuring the
/// length matches the actual allocated size.
///
/// Mitigation: Only use `from_raw_parts` when you can guarantee the pointer and length are valid.
/// Document the safety invariants. Prefer safe Rust slicing when possible. Use Miri to detect
/// invalid slice creation.

pub fn e1609_invalid_slice() {
    let data = [1, 2, 3];
    let ptr = data.as_ptr();

    // PROBLEM E1003: Direct use of unsafe code
    unsafe {
        // PROBLEM E1004: No safety documentation
        // PROBLEM E1609: Creating slice with length beyond actual data
        let _slice = std::slice::from_raw_parts(ptr, 100);
    }
}

pub fn e1609_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
