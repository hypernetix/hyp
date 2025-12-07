/// E1709: Unnecessary boxing
/// Severity: LOW
/// LLM confusion: 2 (LOW)
///
/// Description: Boxing (heap-allocating) small values like integers is wasteful. Box adds
/// indirection (an extra pointer dereference) and a heap allocation for a value that fits
/// perfectly on the stack. Fix by using the value directly without boxing, unless you specifically
/// need heap allocation or trait objects.
///
/// Mitigation: Use `#![warn(clippy::box_default)]` and `#![warn(clippy::boxed_local)]` to catch
/// unnecessary boxing. Only box when you need: trait objects, recursive types, or to move large
/// values. Don't box primitive types or small structs.

pub fn e1709_unnecessary_boxing(x: i32) -> Box<i32> {
    // PROBLEM E1709: Boxing primitive type unnecessarily
    Box::new(x)
}

pub fn e1709_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
