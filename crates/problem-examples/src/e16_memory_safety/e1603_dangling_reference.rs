/// E1603: Dangling reference
/// Severity: HIGH
/// LLM confusion: 4 (HIGH)
///
/// Description: A dangling reference points to memory that no longer contains valid data. This
/// code tries to return a reference to a local variable, which will be destroyed when the function
/// returns. The reference would point to invalid stack memory. Rust's compiler prevents this at
/// compile time. Fix by returning owned data or using proper lifetime annotations.
///
/// Mitigation: Rust's borrow checker prevents dangling references at compile time. If you get
/// lifetime errors, don't fight the compiler - restructure to return owned data or use proper
/// lifetime relationships. Never try to return references to local variables.

pub fn e1603_dangling_reference() -> &'static i32 {
    let x = 42;
    // PROBLEM E1603: Returning reference to local variable (won't compile)
    // &x
    &0 // Placeholder to make it compile
}

pub fn e1603_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
