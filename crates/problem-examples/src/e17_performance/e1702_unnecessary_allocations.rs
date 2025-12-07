/// E1702: Unnecessary allocations
/// Severity: LOW
/// LLM confusion: 2 (LOW)
///
/// Description: Allocating memory (like creating Strings) inside tight loops is slow because
/// allocation is expensive. This code allocates a new String on every iteration. Fix by reusing
/// allocations - create the String once and reuse it, or use `with_capacity()` to pre-allocate
/// the Vec to avoid repeated reallocations.
///
/// Mitigation: Use `Vec::with_capacity()` to pre-allocate when you know the size. Reuse String
/// buffers with `.clear()` instead of creating new ones. Use `format_args!` for formatting without
/// allocation. Profile to identify allocation hotspots.

pub fn e1702_unnecessary_allocations(count: usize) -> Vec<String> {
    let mut result = Vec::new();
    for i in 0..count {
        // PROBLEM E1702: Allocating String in loop instead of reusing
        let s = format!("Item {}", i);
        result.push(s);
    }
    result
}

pub fn e1702_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
