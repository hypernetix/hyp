/// E1303: Ignoring errors with let _ =
/// Severity: HIGH
/// LLM confusion: 3 (MED)
///
/// Description: Using `let _ =` explicitly throws away the result of an operation, including any
/// errors that occurred. This is like catching an exception and doing nothing with it - critical
/// operations might fail silently, making bugs nearly impossible to track down. Imagine deleting
/// a file and not checking if it actually got deleted. Fix by properly handling the error with
/// if/match statements, or at minimum logging what went wrong before ignoring it.
///
/// Mitigation: Use `#![warn(clippy::let_underscore_must_use)]` to catch this pattern. If you
/// truly need to ignore an error, use `.ok()` or add a comment explaining why. Better yet,
/// log the error before ignoring it.

pub fn e1303_ignored_errors() {
    let _ = std::fs::remove_file("temp.txt"); // PROBLEM: Error silently ignored
}

pub fn e1303_entry() -> Result<(), Box<dyn std::error::Error>> {
    e1303_ignored_errors();
    Ok(())
}
