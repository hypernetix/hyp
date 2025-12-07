/// E1604: Buffer overflow
/// Severity: HIGH
/// LLM confusion: 4 (HIGH)
///
/// Description: Buffer overflow occurs when you write beyond the bounds of allocated memory.
/// In safe Rust, array indexing is bounds-checked and will panic. In unsafe code with raw
/// pointers, you can write past the end of a buffer, corrupting adjacent memory. Fix by always
/// validating indices and buffer sizes before pointer arithmetic.
///
/// Mitigation: Avoid pointer arithmetic when possible - use safe Rust slices. If pointer
/// arithmetic is necessary, carefully validate all offsets. Use `ptr.add()` instead of manual
/// arithmetic. Test with tools like AddressSanitizer and Miri to detect buffer overflows.

pub fn e1604_buffer_overflow() {
    let mut buffer = [0u8; 10];
    let index = 15;

    // PROBLEM E1604: No bounds checking (will panic in safe Rust)
    // buffer[index] = 42;

    // PROBLEM E1003: Direct use of unsafe code
    unsafe {
        // PROBLEM E1004: No safety documentation
        // PROBLEM E1604: Actual buffer overflow in unsafe code
        let ptr = buffer.as_mut_ptr();
        *ptr.add(index) = 42;
    }
}

pub fn e1604_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
