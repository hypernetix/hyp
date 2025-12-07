/// E1102: Deeply nested logic in loops and conditions
/// Severity: MED
/// LLM confusion: 4 (HIGH)
///
/// Description: This code has deeply nested loops and conditions that make it extremely difficult
/// to understand what it does. Cognitive complexity measures how hard code is to understand, not
/// just how many branches exist. Fix by flattening nested structures, extracting inner loops into
/// separate functions, and using iterator methods to express intent more clearly.
///
/// Mitigation: Enable `#![warn(clippy::cognitive_complexity)]`. Break down nested loops into
/// separate functions with descriptive names. Use iterator combinators like `filter_map` and
/// `flat_map` to reduce nesting.

pub fn e1102_deep_nested_logic_in_loops(items: Vec<i32>) -> Vec<i32> {
    // PROBLEM E1102: Deeply nested logic that's hard to understand
    let mut result = Vec::new();
    for item in items {
        if item > 0 {
            for i in 0..item {
                if i % 2 == 0 {
                    for j in 0..i {
                        if j % 3 == 0 {
                            result.push(j);
                        }
                    }
                }
            }
        }
    }
    result
}

// Public entry function that prepares arguments and calls the actual problem function
pub fn e1102_entry() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running E1102: Deeply nested logic in loops and conditions");
    let _ = e1102_deep_nested_logic_in_loops(vec![1, 2, 3]);
    Ok(())
}
