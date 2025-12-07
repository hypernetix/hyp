/// E1010: Mutable static without synchronization
/// Severity: HIGH
/// LLM confusion: 4 (HIGH)
///
/// Description: Mutable global variables (static mut) can be accessed from any thread without
/// synchronization, causing data races. If two threads read and write the same global variable
/// simultaneously, the results are unpredictable - you might get corrupted data, crashes, or
/// security vulnerabilities. This is why accessing mutable statics requires unsafe code - it's
/// inherently dangerous. Use thread-safe alternatives instead.
///
/// Mitigation: Avoid mutable statics entirely. Use `static` with `Mutex<T>` or `RwLock<T>` for
/// thread-safe global state. Use `AtomicXxx` types for simple counters/flags. Use `thread_local!`
/// for thread-local state. If mutable statics are necessary, document the synchronization strategy.

static mut COUNTER: i32 = 0;

pub fn e1010_ffi_no_error_handling() {
    // PROBLEM E1003: Direct use of unsafe code
    unsafe {
        // PROBLEM E1004: No safety documentation
        // PROBLEM E1010: Accessing mutable static without synchronization
        COUNTER += 1;
    }
}

pub fn e1010_entry() -> Result<(), Box<dyn std::error::Error>> {
    e1010_ffi_no_error_handling();
    Ok(())
}
