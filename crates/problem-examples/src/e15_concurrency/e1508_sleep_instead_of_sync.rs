/// E1508: Sleep instead of synchronization
/// Severity: LOW
/// LLM confusion: 2 (LOW)
///
/// Description: Using `thread::sleep` to wait for another thread is unreliable and inefficient.
/// The sleep duration is arbitrary - too short and you'll read before the other thread finishes,
/// too long and you waste time. Fix by using proper synchronization primitives like channels,
/// condition variables, or joining the thread.
///
/// Mitigation: Use `thread::join()` to wait for thread completion. Use channels (`mpsc::channel`)
/// to communicate between threads. Use condition variables (`Condvar`) for complex waiting
/// scenarios. Never rely on sleep for synchronization - it's a code smell.

pub fn e1508_sleep_instead_of_sync() {
    let data = std::sync::Arc::new(std::sync::Mutex::new(0));
    let data_clone = data.clone();

    std::thread::spawn(move || {
        // PROBLEM E1002: direct unwrap/expect
        let mut guard = data_clone.lock().unwrap();
        *guard = 42;
    });

    // PROBLEM E1508: Using sleep instead of proper synchronization
    std::thread::sleep(std::time::Duration::from_millis(100));
    // PROBLEM E1002: direct unwrap/expect
    let value = *data.lock().unwrap();
}

pub fn e1508_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
