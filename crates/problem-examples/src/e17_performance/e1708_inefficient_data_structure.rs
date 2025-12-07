/// E1708: Inefficient data structure
/// Severity: LOW
/// LLM confusion: 2 (LOW)
///
/// Description: Different data structures have different performance characteristics. Vec is
/// optimized for indexed access and iteration, but checking membership with `.contains()` is
/// O(N) - it scans the entire vector. For membership testing, HashSet is O(1). Fix by using
/// HashSet when you need fast membership testing.
///
/// Mitigation: Use HashSet for membership testing, HashMap for key-value lookups, BTreeSet/BTreeMap
/// for sorted data, Vec for indexed access. Understand the time complexity of operations on each
/// data structure. Profile to verify your choice improves performance.

#[allow(clippy::ptr_arg)]
pub fn e1708_inefficient_data_structure(data: &Vec<i32>, target: i32) -> bool {
    // PROBLEM E1708: Using Vec for membership testing (should use HashSet)
    data.contains(&target)
}

pub fn e1708_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
