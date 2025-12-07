/// E1809: Fallible new()
/// Severity: MED
/// LLM confusion: 2 (LOW)
///
/// Description: By convention, `new()` should be infallible (never fail). If construction can
/// fail, use a different name like `try_new()`, `from_*()`, or `with_*()`. This `new()` panics
/// on invalid input, which violates user expectations. Fix by returning `Result` and renaming
/// to `try_new()` or similar.
///
/// Mitigation: Make `new()` infallible - it should always succeed. Use `try_new()` or `from_*()`
/// for fallible construction that returns `Result`. Use `default()` for default construction.
/// Follow Rust API conventions to meet user expectations.

pub struct E1809FallibleNew;

impl E1809FallibleNew {
    // PROBLEM E1809: new() should not fail, use try_new() or from_*()
    pub fn new(value: &str) -> Self {
        if value.is_empty() {
            print!("going to panic now");
            // PROBLEM E1001: Direct panic in production code
            panic!("Value cannot be empty");
        }
        Self
    }
}

pub fn e1809_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
