/// E1710: Large stack allocation
/// Severity: MED
/// LLM confusion: 3 (MED)
///
/// Description: The stack has limited size (typically a few MB). Allocating large arrays on the
/// stack can cause stack overflow, especially in recursive functions or deeply nested calls. This
/// allocates 1MB on the stack, which is risky. Fix by using heap allocation (Vec or Box) for
/// large data structures.
///
/// Mitigation: Use `Vec` or `Box` for large allocations instead of stack arrays. Keep stack
/// allocations small (< 1KB is safe, > 100KB is risky). Use `#![warn(clippy::large_stack_arrays)]`
/// to detect large stack allocations. Be especially careful in recursive functions.

pub fn e1710_large_stack_allocation() {
    // PROBLEM E1710: Allocating large array on stack
    let _large_array = [0u8; 1024 * 1024]; // 1MB on stack
}

pub fn e1710_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
