/// E1504: Interior mutability race
/// Severity: HIGH
/// LLM confusion: 4 (HIGH)
///
/// Description: Interior mutability types like Cell and RefCell allow mutation through shared
/// references, but they're not thread-safe (not Sync). Cell doesn't use any synchronization,
/// so sharing it across threads would cause data races. Fix by using thread-safe alternatives
/// like Mutex, RwLock, or atomic types.
///
/// Mitigation: Use `Mutex<T>` or `RwLock<T>` for thread-safe interior mutability. Use atomic
/// types (`AtomicI32`, etc.) for simple values. The compiler prevents this at compile time -
/// Cell and RefCell are not Sync. Understand the difference between Send and Sync traits.

pub fn e1504_interior_mutability_race() {
    use std::cell::Cell;

    let data = Cell::new(0);
    // PROBLEM E1504: Cell is not Sync, sharing across threads is unsafe
    // This won't compile but illustrates the issue
}

pub fn e1504_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
