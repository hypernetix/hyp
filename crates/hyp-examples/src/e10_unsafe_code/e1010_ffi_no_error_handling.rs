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

pub fn e1010_bad_ffi_no_error_handling() {
    // PROBLEM E1003: Direct use of unsafe code
    unsafe {
        // PROBLEM E1004: No safety documentation
        // PROBLEM E1010: Accessing mutable static without synchronization
        COUNTER += 1;
    }
}

pub fn e1010_entry() -> Result<(), Box<dyn std::error::Error>> {
    e1010_bad_ffi_no_error_handling();
    Ok(())
}

// ============================================================================
// GOOD EXAMPLES - Proper alternatives
// ============================================================================

use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Mutex;

/// GOOD: Use atomic types for simple counters
static GOOD_COUNTER: AtomicI32 = AtomicI32::new(0);

pub fn e1010_good_atomic() {
    GOOD_COUNTER.fetch_add(1, Ordering::SeqCst);
    let value = GOOD_COUNTER.load(Ordering::SeqCst);
    println!("Counter: {}", value);
}

/// GOOD: Use Mutex for complex state
use std::sync::LazyLock;

static GOOD_STATE: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));

pub fn e1010_good_mutex() {
    let mut guard = GOOD_STATE.lock().unwrap();
    *guard += 1;
    println!("State: {}", *guard);
}

// GOOD: Use thread_local for per-thread state
thread_local! {
    static THREAD_COUNTER: std::cell::Cell<i32> = const { std::cell::Cell::new(0) };
}

pub fn e1010_good_thread_local() {
    THREAD_COUNTER.with(|c| {
        c.set(c.get() + 1);
        println!("Thread counter: {}", c.get());
    });
}

/// GOOD: Use const for immutable globals
const MAX_VALUE: i32 = 100;

pub fn e1010_good_const() -> i32 {
    MAX_VALUE // No synchronization needed for immutable data
}

// ============================================================================
// GOOD EXAMPLES unit tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn e1010_good_atomic_increments() {
        e1010_good_atomic();
    }

    #[test]
    fn e1010_good_mutex_updates_state() {
        e1010_good_mutex();
    }

    #[test]
    fn e1010_good_const_returns_max() {
        assert_eq!(e1010_good_const(), 100);
    }
}
