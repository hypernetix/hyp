/// E1602: Use-after-free
/// Severity: HIGH
/// LLM confusion: 5 (HIGHEST)
///
/// Description: Use-after-free occurs when you access memory after it has been deallocated.
/// This code keeps a raw pointer to a vector, then drops the vector (freeing its memory), then
/// dereferences the pointer. This accesses freed memory, causing undefined behavior - the memory
/// might be reused for something else. Fix by ensuring pointers don't outlive the data they point to.
///
/// Mitigation: Avoid raw pointers when possible - use references which are lifetime-checked. If
/// raw pointers are necessary, carefully track the lifetime of pointed-to data. Use tools like
/// Miri to detect use-after-free bugs. Never dereference a pointer after its data is dropped.

pub fn e1602_use_after_free() {
    let data = vec![1, 2, 3];
    let ptr = data.as_ptr();
    drop(data);

    // PROBLEM E1003: Direct use of unsafe code
    unsafe {
        // PROBLEM E1004: No safety documentation
        // PROBLEM E1602: Accessing freed memory
        let _value = *ptr;
    }
}

pub fn e1602_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
