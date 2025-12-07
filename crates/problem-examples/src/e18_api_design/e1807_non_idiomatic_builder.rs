/// E1807: Non-idiomatic builder
/// Severity: LOW
/// LLM confusion: 2 (LOW)
///
/// Description: The builder pattern in Rust typically has methods return `self` to allow method
/// chaining like `builder.set_a(1).set_b(2).build()`. This builder doesn't return self, so you
/// can't chain calls. Fix by returning `self` (or `&mut self`) from builder methods to enable
/// chaining.
///
/// Mitigation: Builder methods should return `Self` or `&mut Self` to enable chaining. Use the
/// `typed-builder` crate for automatic builder generation. Follow the pattern: `pub fn field(mut self, value: T) -> Self { self.field = value; self }`.

pub struct E1807Builder {
    value: i32,
}

impl E1807Builder {
    // PROBLEM E1807: Builder methods don't return self
    pub fn set_value(&mut self, value: i32) {
        self.value = value;
    }
}

pub fn e1807_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
