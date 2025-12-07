/// E1101: High cyclomatic complexity
/// Severity: MED
/// LLM confusion: 3 (MED)
///
/// Description: This function has too many decision points (if/else branches), making it hard to
/// understand, test, and maintain. High cyclomatic complexity means there are many different paths
/// through the code. Fix by breaking the function into smaller, focused functions or using lookup
/// tables/pattern matching to simplify the logic.
///
/// Mitigation: Use `#![warn(clippy::cognitive_complexity)]` to detect overly complex functions.
/// Refactor by extracting nested conditions into separate helper functions. Consider using early
/// returns to reduce nesting depth.

#[allow(clippy::needless_return, clippy::collapsible_else_if)]
pub fn e1101_high_cyclomatic_complexity(x: i32, y: i32, z: i32) -> i32 {
    // PROBLEM E1101: Too many branches (cyclomatic complexity > 10)
    if x > 0 {
        if y > 0 {
            if z > 0 {
                return x + y + z;
            } else if z < -10 {
                if z < -20 {
                    return x + y;
                }
            } else {
                println!("z is between -10 and 0");
            }
        } else if y < -5 {
            if x > 10 {
                return x - y;
            } else {
                return y - x;
            }
        } else {
            if z > 0 {
                return x * z;
            }
        }
    } else if x < -5 {
        if y > 0 {
            if z > 0 {
                return y + z;
            } else if z < -10 {
                return y - z;
            }
        } else {
            return x * y;
        }
    } else {
        if y > 0 && z > 0 {
            return y + z;
        } else if y < 0 && z < 0 {
            return y * z;
        }
    }

    // Default case
    x + y + z
}

pub fn e1101_entry() -> Result<(), Box<dyn std::error::Error>> {
    let _ = e1101_high_cyclomatic_complexity(5, 3, 2);
    Ok(())
}
