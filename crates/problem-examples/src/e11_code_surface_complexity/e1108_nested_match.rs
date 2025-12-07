/// E1108: Deeply nested match expressions
/// Severity: MED
/// LLM confusion: 3 (MED)
///
/// Description: This code has deeply nested pattern matching (like nested switch statements),
/// making it hard to follow the logic flow. Each level of nesting increases cognitive load.
/// Fix by flattening the match expressions using early returns, combining patterns, or
/// extracting nested matches into separate functions.
///
/// Mitigation: Use `#![warn(clippy::cognitive_complexity)]` to detect overly complex patterns.
/// Flatten nested matches using tuple patterns like `match (opt1, opt2)` or use `if let` chains.
/// Extract complex match arms into helper functions.

pub fn e1108_nested_match(opt1: Option<i32>, opt2: Option<i32>) -> i32 {
    // PROBLEM E1108: Deeply nested pattern matching
    match opt1 {
        Some(x) => match opt2 {
            Some(y) => match x.cmp(&y) {
                std::cmp::Ordering::Less => x,
                std::cmp::Ordering::Equal => match x {
                    0 => 1,
                    _ => x,
                },
                std::cmp::Ordering::Greater => y,
            },
            None => x,
        },
        None => 0,
    }
}

pub fn e1108_entry() -> Result<(), Box<dyn std::error::Error>> {
    let _ = e1108_nested_match(Some(42), Some(1));
    Ok(())
}
