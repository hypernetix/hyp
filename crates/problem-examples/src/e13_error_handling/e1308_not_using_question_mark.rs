/// E1308: Not using ? operator when appropriate
/// Severity: LOW
/// LLM confusion: 2 (LOW)
///
/// Description: This code manually handles errors with verbose match/if statements instead of
/// using the `?` operator, which is a shorthand for "if this fails, return the error to my caller".
/// The manual approach is much more verbose and harder to read. It's like writing out a full
/// try/catch block when you could just use a throws declaration. Fix by replacing manual error
/// matching with the `?` operator for cleaner, more idiomatic code.
///
/// Mitigation: Use `#![warn(clippy::manual_map)]` to detect patterns that could use `?`. Learn
/// the `?` operator - it's the idiomatic way to handle errors in Rust. It automatically converts
/// error types when needed and makes code much cleaner.

#[allow(clippy::question_mark)]
pub fn e1308_not_using_question_mark() -> Result<i32, std::io::Error> {
    // PROBLEM E1308: Verbose error handling instead of ?
    let data = match std::fs::read_to_string("file.txt") {
        Ok(d) => d,
        Err(e) => return Err(e),
    };
    Ok(data.len() as i32)
}

pub fn e1308_entry() -> Result<(), Box<dyn std::error::Error>> {
    let _ = e1308_not_using_question_mark();
    Ok(())
}
