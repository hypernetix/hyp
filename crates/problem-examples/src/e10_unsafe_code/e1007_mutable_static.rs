/// E1007: Dereferencing null pointer
/// Severity: HIGH
/// LLM confusion: 4 (HIGH)
///
/// Description: Dereferencing a null pointer (address 0) is undefined behavior and will typically
/// crash the program immediately. This is one of the most common bugs in C/C++ code. In Rust, you
/// have to explicitly use unsafe code to create and dereference null pointers, which is why this
/// is a serious error - you're bypassing safety checks to do something dangerous. Never dereference
/// a pointer without checking if it's null first.
///
/// Mitigation: Never create null pointers in safe code. If working with FFI that might return null,
/// check for null before dereferencing. Use `Option<NonNull<T>>` to make nullability explicit.
/// Use `ptr.is_null()` to check before dereferencing. Prefer safe Rust references over raw pointers.

pub fn e1007_bad_mutable_static(input: i32) {
    let ptr: *const i32 = std::ptr::null();

    if input > 0 {
        // PROBLEM E1003: Direct use of unsafe code
        unsafe {
            // PROBLEM E1004: No safety documentation
            // PROBLEM E1007: Dereferencing null pointer (undefined behavior)
            let _value = *ptr;
        }
    }
}

pub fn e1007_entry() -> Result<(), Box<dyn std::error::Error>> {
    e1007_bad_mutable_static(0);
    Ok(())
}

// ============================================================================
// GOOD EXAMPLES - Proper alternatives
// ============================================================================

use std::ptr::NonNull;

/// GOOD: Use Option<&T> instead of nullable pointers
pub fn e1007_good_use_option(value: Option<&i32>) -> i32 {
    match value {
        Some(v) => *v,
        None => 0, // Handle the null case explicitly
    }
}

/// GOOD: Use NonNull to guarantee non-null pointers
pub fn e1007_good_use_nonnull(ptr: NonNull<i32>) -> i32 {
    // SAFETY: NonNull guarantees the pointer is not null
    unsafe { *ptr.as_ptr() }
}

/// GOOD: Check for null before dereferencing
pub fn e1007_good_null_check(ptr: *const i32) -> Option<i32> {
    if ptr.is_null() {
        return None;
    }
    // SAFETY: We just checked that ptr is not null
    unsafe { Some(*ptr) }
}

/// GOOD: Use references instead of raw pointers
pub fn e1007_good_use_references(value: &i32) -> i32 {
    *value // References are never null in safe Rust
}

// ============================================================================
// GOOD EXAMPLES unit tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr::NonNull;

    #[test]
    fn e1007_good_use_option_handles_none() {
        assert_eq!(e1007_good_use_option(None), 0);
    }

    #[test]
    fn e1007_good_use_nonnull_reads_value() {
        let value = Box::new(10);
        let ptr = NonNull::from(value.as_ref());
        assert_eq!(e1007_good_use_nonnull(ptr), 10);
    }

    #[test]
    fn e1007_good_null_check_detects_null() {
        let ptr: *const i32 = std::ptr::null();
        assert_eq!(e1007_good_null_check(ptr), None);
    }

    #[test]
    fn e1007_good_use_references_reads_through_reference() {
        let value = 7;
        assert_eq!(e1007_good_use_references(&value), 7);
    }
}
