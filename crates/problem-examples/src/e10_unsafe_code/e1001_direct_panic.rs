/// E1001: Direct call of panic() in production code
/// Severity: HIGH
/// LLM confusion: 2 (LOW)
///
/// Description: Calling panic!() directly in production code immediately crashes the entire program
/// when the condition is met. This is like throwing an unhandled exception that terminates the
/// application - there's no way for callers to recover or handle the error gracefully. Instead of
/// crashing, return a Result type so callers can decide how to handle the error (retry, log, use
/// a default value, etc.).
///
/// Mitigation: Use `#![warn(clippy::panic)]` to catch direct panic calls. Return `Result<T, E>`
/// instead of panicking. Reserve panic!() only for truly unrecoverable errors or bugs that should
/// never happen. Use `expect()` with a clear message for cases that should be impossible.

pub fn e1001_direct_panic(value: i32) {
    if value > 40 {
        // PROBLEM E1001: Direct panic in production code
        panic!("Value too large!");
    }
}

pub fn e1001_entry() -> Result<(), Box<dyn std::error::Error>> {
    e1001_direct_panic(0);
    Ok(())
}
