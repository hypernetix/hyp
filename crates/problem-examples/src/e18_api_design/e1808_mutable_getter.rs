/// E1808: Mutable getter
/// Severity: MED
/// LLM confusion: 2 (LOW)
///
/// Description: Returning a mutable reference to internal data breaks encapsulation - callers can
/// modify the internal state in ways that violate invariants. This getter returns `&mut Vec`,
/// allowing callers to clear it, add invalid data, etc. Fix by providing specific methods for
/// allowed operations instead of exposing mutable internals.
///
/// Mitigation: Avoid mutable getters. Provide specific methods for allowed mutations (e.g.,
/// `add_item()`, `remove_item()`). If you must expose collections, return iterators or slices
/// instead of mutable references. Use `#![warn(clippy::mut_from_ref)]` to catch suspicious patterns.

pub struct E1808MutableGetter {
    data: Vec<i32>,
}

impl E1808MutableGetter {
    // PROBLEM E1808: Returning mutable reference breaks encapsulation
    pub fn get_data_mut(&mut self) -> &mut Vec<i32> {
        &mut self.data
    }
}

pub fn e1808_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
