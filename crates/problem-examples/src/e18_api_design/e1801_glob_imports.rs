/// E1801: Glob imports
/// Severity: LOW
/// LLM confusion: 2 (LOW)
///
/// Description: Glob imports (`use module::*`) bring all public items from a module into scope,
/// making it unclear where each item comes from. This hurts code readability and can cause name
/// conflicts. Fix by importing items explicitly by name, which makes dependencies clear and
/// prevents accidental name collisions.
///
/// Mitigation: Use `#![warn(clippy::wildcard_imports)]` to catch glob imports. Import items
/// explicitly: `use std::collections::{HashMap, HashSet}`. Glob imports are acceptable for preludes
/// and test modules, but avoid them in production code.

#[allow(clippy::module_inception)]
pub mod e1801_glob_imports {
    // PROBLEM E1801: Glob imports make it unclear where items come from
    use std::collections::*;

    pub fn e1801_glob_imports() {
        let map: HashMap<i32, i32> = HashMap::new();
        print!("{}", map.len());
    }
}

pub fn e1801_entry() -> Result<(), Box<dyn std::error::Error>> {
    e1801_glob_imports::e1801_glob_imports();
    Ok(())
}
