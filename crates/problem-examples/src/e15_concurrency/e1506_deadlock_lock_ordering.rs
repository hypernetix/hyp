/// E1506: Deadlock from lock ordering
/// Severity: HIGH
/// LLM confusion: 3 (MED)
///
/// Description: Deadlock occurs when two threads each hold a lock and wait for the other's lock,
/// causing both to wait forever. This happens when locks are acquired in different orders. Thread 1
/// locks A then B, while Thread 2 locks B then A - they get stuck. Fix by always acquiring locks
/// in the same order, or use try_lock with timeouts.
///
/// Mitigation: Establish a global lock ordering and always acquire locks in that order. Use
/// `try_lock()` with timeouts to detect potential deadlocks. Consider using a single lock for
/// related data. Use deadlock detection tools like `parking_lot` which has better diagnostics.

pub fn e1506_deadlock_lock_ordering(
    mutex1: std::sync::Arc<std::sync::Mutex<i32>>,
    mutex2: std::sync::Arc<std::sync::Mutex<i32>>,
) {
    // Thread 1
    let m1 = mutex1.clone();
    let m2 = mutex2.clone();
    std::thread::spawn(move || {
        // PROBLEM E1002: direct unwrap/expect
        let _g1 = m1.lock().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        // PROBLEM E1002: direct unwrap/expect
        let _g2 = m2.lock().unwrap(); // PROBLEM: Can deadlock
    });

    // Thread 2
    // PROBLEM E1002: direct unwrap/expect
    let _g2 = mutex2.lock().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(10));
    // PROBLEM E1002: direct unwrap/expect
    let _g1 = mutex1.lock().unwrap(); // PROBLEM: Opposite order
}

pub fn e1506_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
