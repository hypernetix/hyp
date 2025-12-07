/// E1610: Unaligned dereference
/// Severity: HIGH
/// LLM confusion: 4 (HIGH)
///
/// Description: Dereferencing an unaligned pointer causes undefined behavior on many architectures.
/// A u64 typically requires 8-byte alignment, but this code creates a u64 pointer at offset 1
/// (not aligned to 8 bytes). Reading from it can crash or return garbage. Fix by ensuring pointers
/// are properly aligned, or use `read_unaligned()` for unaligned access.
///
/// Mitigation: Use `ptr.read_unaligned()` for potentially unaligned reads. Check alignment with
/// `ptr.align_offset()`. Use `#[repr(packed)]` carefully as it creates unaligned fields. Understand
/// your target architecture's alignment requirements.

pub fn e1610_unaligned_deref() {
    let data = [0u8; 10];
    let ptr = data.as_ptr();

    // PROBLEM E1003: Direct use of unsafe code
    unsafe {
        // PROBLEM E1004: No safety documentation
        // PROBLEM E1610: Casting to u64 pointer may not be aligned
        let ptr64 = ptr.add(1) as *const u64;
        let _value = *ptr64;
    }
}

pub fn e1610_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
