/// E1507: Unsynchronized shared state
/// Severity: HIGH
/// LLM confusion: 4 (HIGH)
///
/// Description: A data race occurs when multiple threads access the same memory location
/// concurrently, at least one is writing, and there's no synchronization. This code uses a
/// static mutable variable accessed from multiple threads without any locks or atomic operations,
/// causing undefined behavior. Fix by using Mutex, RwLock, or atomic types.
///
/// Mitigation: Never use `static mut` in multi-threaded code. Use `static` with `Mutex<T>`,
/// `RwLock<T>`, or atomic types. Use `lazy_static` or `once_cell` for safe static initialization.
/// Rust's type system prevents most data races, but `unsafe` code can bypass these protections.

pub fn e1507_unsynchronized_shared_state() {
    static mut COUNTER: i32 = 0;

    for _ in 0..10 {
        std::thread::spawn(|| {
            // PROBLEM E1003: Direct use of unsafe code
            unsafe {
                // PROBLEM E1004: No safety documentation
                // PROBLEM E1507: Data race on COUNTER
                COUNTER += 1;
            }
        });
    }
}

pub fn e1507_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
