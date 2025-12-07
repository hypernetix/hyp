/// E1110: Deeply nested callbacks/closures
/// Severity: MED
/// LLM confusion: 5 (HIGHEST)
///
/// Description: This code has multiple layers of nested closures (anonymous functions), making it
/// extremely difficult to follow the control flow and understand what data is being captured at
/// each level. Each closure can access variables from outer scopes, creating a complex web of
/// dependencies. It's like having functions within functions within functions, where each inner
/// function can use variables from all the outer functions. This "callback hell" or "pyramid of doom"
/// makes code nearly impossible to debug and maintain. Fix by flattening the structure using
/// async/await, extracting closures into named functions, or using combinator methods.
///
/// Mitigation: Limit closure nesting to 2-3 levels maximum. Extract nested closures into named
/// functions with clear signatures. Use async/await instead of nested callbacks for asynchronous
/// code. Consider using the `?` operator and early returns to reduce nesting. Use combinator
/// methods like `and_then`, `map`, `flat_map` to express intent more clearly.

pub fn e1110_nested_callbacks() {
    let data = [1, 2, 3, 4, 5];

    // PROBLEM E1110: Deeply nested closures (5 levels deep)
    data.iter().for_each(|x| {
        if *x > 0 {
            let result =
                (0..*x)
                    .map(|y| {
                        if y % 2 == 0 {
                            Some(
                                (0..y)
                                    .filter_map(|z| {
                                        if z % 3 == 0 {
                                            Some((0..z).fold(0, |acc, w| {
                                                if w % 2 == 0 {
                                                    acc + w
                                                } else {
                                                    acc
                                                }
                                            }))
                                        } else {
                                            None
                                        }
                                    })
                                    .sum::<i32>(),
                            )
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

            let _sum: i32 = result.iter().filter_map(|x| *x).sum();
        }
    });
}

// PROBLEM E1110: Nested closures with complex variable capture
pub fn e1110_callback_hell() {
    let multiplier = 2;
    let threshold = 10;

    let process = |data: Vec<i32>| {
        data.into_iter()
            .filter(|x| *x > threshold) // Captures threshold
            .map(|x| {
                let doubled = x * multiplier; // Captures multiplier
                (0..doubled)
                    .filter_map(|y| {
                        if y % 2 == 0 {
                            Some(
                                (0..y)
                                    .map(|z| {
                                        z * multiplier // Captures multiplier from outer scope
                                    })
                                    .sum::<i32>(),
                            )
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>()
    };

    let _result = process(vec![5, 10, 15, 20]);
}

pub fn e1110_entry() -> Result<(), Box<dyn std::error::Error>> {
    e1110_nested_callbacks();
    Ok(())
}
