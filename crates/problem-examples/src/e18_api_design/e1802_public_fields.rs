/// E1802: Public fields without validation
/// Severity: MED
/// LLM confusion: 2 (LOW)
///
/// Description: Public fields can be set to any value by external code, bypassing validation.
/// This struct has a public `age` field that could be set to negative values, and an `email`
/// field that could be set to invalid email addresses. Fix by making fields private and providing
/// getter/setter methods that validate inputs.
///
/// Mitigation: Make fields private by default. Provide getter methods for read access and setter
/// methods (or builder pattern) for write access with validation. Use newtype patterns for values
/// that need validation. Consider using the `typed-builder` crate for complex construction.

pub struct E1802PublicFields {
    // PROBLEM E1802: Public fields can be set to invalid values
    pub age: i32,      // Should be validated to be >= 0
    pub email: String, // Should be validated
}

pub fn e1802_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
