/// E1006: Transmute without size/alignment checks
/// Severity: HIGH
/// LLM confusion: 5 (HIGHEST)
///
/// Description: Transmute reinterprets the raw bits of one type as another type without any
/// validation. This is extremely dangerous because it bypasses all type safety - you're telling
/// the compiler "trust me, these bytes represent this other type" without checking if that's true.
/// If the types have different sizes or alignment requirements, this causes undefined behavior.
/// It's like casting a pointer to any type in C without checking - it might work, might crash,
/// or might silently corrupt data.
///
/// Mitigation: Avoid transmute unless absolutely necessary. Use safe conversion methods like
/// `from_ne_bytes()`, `to_ne_bytes()`, or implement proper conversion traits. If transmute is
/// needed, add compile-time size/alignment assertions and extensive safety documentation.
/// Use `#![warn(clippy::transmute_ptr_to_ptr)]` to catch suspicious transmutes.

#[allow(unnecessary_transmutes)]
pub fn e1006_unsafe_transmute() {
    let x: u32 = 42;

    // PROBLEM E1003: Direct use of unsafe code
    unsafe {
        // PROBLEM E1004: No safety documentation
        // PROBLEM E1006: transmute without verifying size/alignment compatibility
        let _y: f32 = std::mem::transmute(x);
    }
}

pub fn e1006_entry() -> Result<(), Box<dyn std::error::Error>> {
    e1006_unsafe_transmute();
    Ok(())
}
