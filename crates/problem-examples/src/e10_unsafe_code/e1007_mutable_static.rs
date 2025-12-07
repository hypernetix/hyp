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

pub fn e1007_mutable_static(input: i32) {
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
    e1007_mutable_static(0);
    Ok(())
}
