/// E1302: Constructors returning bare values instead of Result
/// Severity: MED
/// LLM confusion: 3 (MED)
///
/// Description: This constructor (a function that creates a new instance) can fail but returns
/// a plain value instead of a Result. This forces the constructor to panic when it fails,
/// which crashes the program. Fix by returning `Result<Self, Error>` from constructors that
/// can fail, allowing callers to handle errors gracefully.
///
/// Mitigation: Use `#![warn(clippy::new_without_default)]` and follow the convention that `new()`
/// should never fail. For fallible construction, use `try_new()`, `from_*()`, or similar names
/// that signal the operation can fail.

pub struct Config {
    pub value: i32,
}

impl Config {
    // PROBLEM E1302: Constructor can fail but doesn't return Result
    pub fn new(s: &str) -> Self {
        // PROBLEM E1002: direct unwrap/expect
        let value = s.parse().unwrap(); // Can panic!
        Self { value }
    }
}

pub fn e1302_entry() -> Result<(), Box<dyn std::error::Error>> {
    let _ = Config::new("42");
    Ok(())
}
