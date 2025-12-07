/// E1309: Panic in Drop implementation
/// Severity: HIGH
/// LLM confusion: 4 (HIGH)
///
/// Description: The Drop trait runs cleanup code when a value is destroyed (like a destructor or
/// finally block). Panicking (crashing) in Drop is extremely dangerous because if Drop runs while
/// already handling another crash, it causes a double-crash which immediately terminates the entire
/// program with no chance to recover. It's like throwing an exception in a finally block while
/// already unwinding from another exception. Fix by handling errors in Drop gracefully - log them
/// instead of crashing.
///
/// Mitigation: Never panic in Drop implementations. Log errors instead, or store them for later
/// retrieval. Use `std::panic::catch_unwind` if you must handle panics. Consider providing an
/// explicit cleanup method for fallible operations.

pub struct PanickingDrop {
    value: i32,
}

impl Drop for PanickingDrop {
    fn drop(&mut self) {
        if self.value > 0 {
            // PROBLEM E1001: Direct panic in production code
            panic!("PROBLEM: Panic in Drop can cause double panic");
        }
    }
}

pub fn e1309_entry() -> Result<(), Box<dyn std::error::Error>> {
    let _resource = PanickingDrop { value: 0 };
    Ok(())
}
