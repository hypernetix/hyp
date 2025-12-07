/// E1502: Lock held across await
/// Severity: HIGH
/// LLM confusion: 4 (HIGH)
///
/// Description: In async code, holding a lock (Mutex guard) across an `.await` point is dangerous
/// because the task might be moved to a different thread while waiting. This can cause deadlocks
/// or prevent other tasks from accessing the locked data. Fix by dropping the lock before awaiting,
/// or use async-aware locks like `tokio::sync::Mutex`.
///
/// Mitigation: Use async-aware locks like `tokio::sync::Mutex` or `async_std::sync::Mutex`. Always
/// drop lock guards before `.await` points. Use `#![warn(clippy::await_holding_lock)]` to detect
/// this pattern. Restructure code to minimize lock scope.

#[allow(clippy::await_holding_lock)]
pub async fn e1502_lock_across_await(mutex: std::sync::Arc<std::sync::Mutex<i32>>) {
    // PROBLEM E1002: direct unwrap/expect
    let mut guard = mutex.lock().unwrap();
    *guard += 1;

    // PROBLEM E1502: Holding lock across await point can cause deadlocks
    some_async_operation().await;

    *guard += 1;
}

async fn some_async_operation() {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}

pub fn e1502_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
