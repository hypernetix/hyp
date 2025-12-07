/// E1105: Boolean parameter hell
/// Severity: LOW
/// LLM confusion: 2 (LOW)
///
/// Description: This function takes multiple boolean (true/false) parameters, making it very
/// confusing at the call site. When you see `func(true, false, true, false)`, it's impossible
/// to know what each boolean means without checking the function signature. Fix by using an
/// enum or a configuration struct with named fields instead of boolean parameters.
///
/// Mitigation: Use `#![warn(clippy::fn_params_excessive_bools)]` to detect functions with too
/// many boolean parameters. Replace boolean parameters with enums that have descriptive names,
/// or use a builder pattern with named methods.

pub fn e1105_boolean_params(
    enable_feature_a: bool,
    enable_feature_b: bool,
    enable_feature_c: bool,
    enable_feature_d: bool,
) {
    // PROBLEM E1105: Multiple boolean parameters (confusing at call site)
    if enable_feature_a && enable_feature_b {
        // do something
    }
}

pub fn e1105_entry() -> Result<(), Box<dyn std::error::Error>> {
    e1105_boolean_params(true, false, true, false);
    Ok(())
}
