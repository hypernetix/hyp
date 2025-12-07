/// E1103: Too many function parameters
/// Severity: LOW
/// LLM confusion: 2 (LOW)
///
/// Description: This function takes too many parameters (more than 7), making it difficult to
/// call correctly and hard to remember the parameter order. Functions with many parameters are
/// error-prone because it's easy to pass arguments in the wrong order. Fix by grouping related
/// parameters into a struct or using a builder pattern for complex configurations.
///
/// Mitigation: Use `#![warn(clippy::too_many_arguments)]` to catch functions with too many
/// parameters. Group related parameters into a configuration struct. Consider using the builder
/// pattern for functions that need many optional parameters.

#[allow(clippy::too_many_arguments)]
pub fn e1103_too_many_params(
    param_a: i32,
    param_b: i32,
    param_c: i32,
    param_d: i32,
    param_e: i32,
    param_f: i32,
    param_g: i32,
    param_h: i32,
) -> i32 {
    param_a + param_b + param_c + param_d + param_e + param_f + param_g + param_h
}

pub fn e1103_entry() -> Result<(), Box<dyn std::error::Error>> {
    let _ = e1103_too_many_params(1, 2, 3, 4, 5, 6, 7, 8);
    Ok(())
}
