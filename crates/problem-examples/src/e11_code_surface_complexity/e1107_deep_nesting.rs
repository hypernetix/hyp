/// E1107: Deeply nested conditionals
/// Severity: MED
/// LLM confusion: 3 (MED)
///
/// Description: This function combines multiple layers of `if/else` and `match` in one place,
/// making the control flow hard to follow. It's similar in spirit to E1101/E1102 but focused on
/// nested boolean conditions instead of arithmetic.
///
/// Mitigation: Extract inner branches into helper functions and avoid more than a couple of
/// nested `if/else` levels.

pub fn e1107_deep_nesting(flag_a: bool, flag_b: bool, flag_c: bool, n: i32) -> i32 {
    if flag_a {
        if n > 0 {
            if flag_b {
                if n % 2 == 0 {
                    n * 2
                } else if flag_c {
                    n * 3
                } else {
                    n + 1
                }
            } else {
                if flag_c {
                    if n > 10 {
                        n - 1
                    } else {
                        n + 10
                    }
                } else {
                    0
                }
            }
        } else {
            if flag_b && flag_c {
                -n
            } else if flag_b {
                n - 5
            } else {
                n
            }
        }
    } else {
        if flag_b {
            if flag_c {
                n * n
            } else {
                n / 2
            }
        } else if flag_c {
            match n {
                0 => 0,
                1..=10 => n + 100,
                _ => n - 100,
            }
        } else {
            -1
        }
    }
}

pub fn e1107_entry() -> Result<(), Box<dyn std::error::Error>> {
    let _ = e1107_deep_nesting(true, false, true, 7);
    Ok(())
}
