/// E1505: Non-Send future
/// Severity: HIGH
/// LLM confusion: 4 (HIGH)
///
/// Description: Async futures must be Send if they need to be moved between threads (which most
/// async runtimes do). Using non-Send types like Rc in an async function makes the entire future
/// non-Send, preventing it from being used with multi-threaded executors. Fix by using Send
/// alternatives like Arc, or ensure the non-Send value is dropped before await points.
///
/// Mitigation: Use `Arc` instead of `Rc` in async code. Drop non-Send values before `.await`
/// points. Use `#![warn(clippy::future_not_send)]` to detect non-Send futures. Test that your
/// futures are Send by spawning them on multi-threaded runtimes.

pub async fn e1505_non_send_future() {
    use std::rc::Rc;
    let data = Rc::new(42);

    // PROBLEM E1505: Future is not Send due to Rc
    some_async_operation().await;
    println!("{}", data);
}

async fn some_async_operation() {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}

pub fn e1505_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
