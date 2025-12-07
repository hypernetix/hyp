/// E1109: Excessive method chaining
/// Severity: LOW
/// LLM confusion: 2 (LOW)
///
/// Description: This code chains too many method calls together in a single expression, making
/// it difficult to debug (you can't easily inspect intermediate values) and hard to understand
/// what transformations are being applied. Fix by breaking the chain into intermediate variables
/// with descriptive names, or limiting chains to 5-7 operations.
///
/// Mitigation: Break long chains into steps with meaningful variable names. This makes debugging
/// easier and improves readability. Consider using intermediate `let` bindings when chains exceed
/// 5-7 operations.

pub fn e1109_excessive_chaining(data: Vec<i32>) -> Vec<String> {
    // PROBLEM E1109: Too many chained methods (10 operations - hard to debug)
    data.iter()
        .filter(|x| **x > 0)
        .map(|x| x * 2)
        .filter(|x| x % 3 == 0)
        .map(|x| x + 1)
        .filter(|x| x % 2 == 0)
        .map(|x| x.to_string())
        .filter(|s| s.len() > 1)
        .map(|s| format!("Value: {}", s))
        .filter(|s| !s.contains('0'))
        .collect()
}

pub fn e1109_entry() -> Result<(), Box<dyn std::error::Error>> {
    let _ = e1109_excessive_chaining(vec![1, 2, 3, 4, 5]);
    Ok(())
}
