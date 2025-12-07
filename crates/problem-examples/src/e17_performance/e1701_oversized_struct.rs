/// E1701: Oversized struct passed by value
/// Severity: LOW
/// LLM confusion: 2 (LOW)
///
/// Description: Passing large structs by value (not by reference) copies all the data, which is
/// slow and uses extra stack space. This struct is 4KB, so every function call copies 4KB of data.
/// Fix by passing large structs by reference (`&OversizedStruct`) instead of by value, or use
/// `Box<OversizedStruct>` to pass a pointer.
///
/// Mitigation: Use `#![warn(clippy::large_types_passed_by_value)]` to detect large types passed
/// by value. Pass structs larger than ~128 bytes by reference. Use `Box` for very large types.
/// Consider if the struct needs to be that large - maybe split it up.

pub struct OversizedStruct {
    // PROBLEM E1701: Large struct (> 1KB) passed by value
    pub data: [u8; 2048],
    pub more_data: [u8; 2048],
}

pub fn e1701_oversized_struct(s: OversizedStruct) {
    // Passing large struct by value is inefficient
}

pub fn e1701_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
