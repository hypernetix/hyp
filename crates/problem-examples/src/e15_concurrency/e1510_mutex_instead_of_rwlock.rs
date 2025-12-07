/// E1510: Mutex instead of RwLock
/// Severity: LOW
/// LLM confusion: 2 (LOW)
///
/// Description: Mutex allows only one reader OR one writer at a time. RwLock allows multiple
/// simultaneous readers but only one writer. For read-heavy workloads, using Mutex is inefficient
/// because readers block each other unnecessarily. Fix by using RwLock when you have many reads
/// and few writes.
///
/// Mitigation: Use `RwLock` for read-heavy workloads - it allows multiple concurrent readers.
/// Use `Mutex` for write-heavy workloads or when simplicity is preferred. Profile to verify that
/// RwLock improves performance - it has higher overhead than Mutex for writes.

pub fn e1510_mutex_instead_of_rwlock() {
    let data = std::sync::Arc::new(std::sync::Mutex::new(vec![1, 2, 3]));

    // PROBLEM E1510: Using Mutex for read-heavy workload (RwLock would be better)
    for _ in 0..10 {
        let data_clone = data.clone();
        std::thread::spawn(move || {
            // PROBLEM E1002: direct unwrap/expect
            let guard = data_clone.lock().unwrap();
            let _sum: i32 = guard.iter().sum();
        });
    }
}

pub fn e1510_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
