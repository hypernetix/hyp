/// E1310: Error context loss
/// Severity: MED
/// LLM confusion: 3 (MED)
///
/// Description: This code discards the original error (using `|_|`) when converting it to a
/// different error type, losing valuable debugging information like which file failed to open,
/// why it failed (permissions? doesn't exist?), etc. It's like catching an exception and throwing
/// a new one with just a generic message, losing the original stack trace and details. Fix by
/// preserving the original error in the new error, either by wrapping it or including its message.
///
/// Mitigation: Use `map_err(|e| ...)` to preserve error context. Use the `anyhow` crate for
/// applications or `thiserror` for libraries to maintain error chains. Never use `|_|` to
/// discard errors - always preserve the original error information.

pub fn e1310_error_context_loss() -> Result<i32, Box<dyn std::error::Error>> {
    // PROBLEM E1310: Original error context is lost
    let data = std::fs::read_to_string("config.txt").map_err(|_| "Failed to read file")?;
    Ok(data.len() as i32)
}

pub fn e1310_entry() -> Result<(), Box<dyn std::error::Error>> {
    let _ = e1310_error_context_loss();
    Ok(())
}
