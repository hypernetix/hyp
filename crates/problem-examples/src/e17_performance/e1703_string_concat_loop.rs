/// E1703: String concatenation in loop
/// Severity: LOW
/// LLM confusion: 2 (LOW)
///
/// Description: Using `+` to concatenate strings in a loop is very inefficient because each `+`
/// creates a new String allocation. With N items, this does O(N²) work because each concatenation
/// copies all previous characters. Fix by using `push_str()` to append to the same String, or
/// use `join()` for collections.
///
/// Mitigation: Use `String::push_str()` or `String::push()` to append to existing strings. Use
/// `[].join()` for joining collections. Pre-allocate with `String::with_capacity()` when you know
/// the final size. Use `#![warn(clippy::string_add_assign)]` to catch `+=` on strings.

pub fn e1703_string_concat_loop(items: &[&str]) -> String {
    let mut result = String::new();
    for item in items {
        // PROBLEM E1703: Inefficient string concatenation
        result = result + item + ", ";
    }
    result
}

pub fn e1703_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
