/// E1503: Lock poisoning mishandled
/// Severity: MED
/// LLM confusion: 3 (MED)
///
/// Description: When a thread panics while holding a Mutex, the Mutex becomes "poisoned" to
/// indicate that the protected data might be in an inconsistent state. Using `.unwrap()` on
/// lock() will panic if the Mutex is poisoned. Fix by handling poisoned locks explicitly with
/// `into_inner()` or by using a different synchronization primitive.
///
/// Mitigation: Handle poisoned locks explicitly: use `lock().unwrap_or_else(|e| e.into_inner())`
/// to recover the data despite poisoning. Consider whether your data can be safely used after a
/// panic. Use RwLock or other primitives if lock poisoning is problematic.

pub fn e1503_lock_poisoning(mutex: std::sync::Arc<std::sync::Mutex<i32>>) {
    // PROBLEM E1503: Not handling poisoned lock properly
    // PROBLEM E1002: direct unwrap/expect
    let mut guard = mutex.lock().unwrap(); // Can panic if poisoned
    *guard += 1;
}

pub fn e1503_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
