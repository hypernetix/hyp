/// E1704: Unnecessary collect()
/// Severity: LOW
/// LLM confusion: 2 (LOW)
///
/// Description: Calling `.collect()` creates a new collection by consuming an iterator. This code
/// collects into a Vec, then immediately creates another iterator from it. The collect is
/// unnecessary - you can chain iterator operations without collecting intermediate results. Fix
/// by removing the unnecessary collect and chaining operations directly.
///
/// Mitigation: Use `#![warn(clippy::needless_collect)]` to detect unnecessary collections. Chain
/// iterator operations instead of collecting intermediate results. Only collect when you actually
/// need the final collection. Iterators are lazy and efficient.

pub fn e1704_unnecessary_collect(data: Vec<i32>) -> i32 {
    // PROBLEM E1704: Collecting when not needed
    data.iter()
        .filter(|x| **x > 0)
        .collect::<Vec<_>>()
        .iter()
        .copied()
        .sum()
}

pub fn e1704_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
