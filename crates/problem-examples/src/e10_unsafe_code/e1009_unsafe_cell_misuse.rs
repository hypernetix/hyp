/// E1009: UnsafeCell misuse and interior mutability violations
/// Severity: HIGH
/// LLM confusion: 5 (HIGHEST)
///
/// Description: UnsafeCell is Rust's primitive for interior mutability, allowing mutation through
/// shared references. However, it provides NO safety guarantees - you must manually ensure that
/// mutable access is exclusive. Using UnsafeCell incorrectly can create multiple mutable aliases
/// to the same data, violating Rust's aliasing rules and causing undefined behavior. This is like
/// having two mutable references to the same memory location, which Rust normally prevents.
///
/// Mitigation: Use safe wrappers like `Cell<T>`, `RefCell<T>`, `Mutex<T>`, or `RwLock<T>` instead
/// of UnsafeCell directly. If you must use UnsafeCell, carefully document your synchronization
/// strategy and ensure exclusive access. Use `#![warn(clippy::undocumented_unsafe_blocks)]` and
/// add detailed safety comments explaining why each unsafe block is sound.
use std::cell::UnsafeCell;

pub struct BadCell<T> {
    value: UnsafeCell<T>,
}

impl<T> BadCell<T> {
    pub fn new(value: T) -> Self {
        BadCell {
            value: UnsafeCell::new(value),
        }
    }

    // PROBLEM E1009: Returns multiple mutable references to the same data!
    // This violates Rust's aliasing rules and causes undefined behavior
    pub fn get_mut(&self) -> &mut T {
        // PROBLEM E1003: Direct use of unsafe code
        unsafe {
            // PROBLEM E1004: No safety documentation
            // DANGER: This creates aliasing mutable references
            &mut *self.value.get()
        }
    }
}

pub fn e1009_unsafe_cell_misuse() {
    let cell = BadCell::new(42);

    // PROBLEM E1009: Two mutable references to the same memory!
    let ref1 = cell.get_mut();
    let ref2 = cell.get_mut();

    // Undefined behavior: modifying through both references
    *ref1 = 100;
    *ref2 = 200;

    // Which value is it? Undefined!
    println!("Value: {}", *ref1);
}

pub fn e1009_entry() -> Result<(), Box<dyn std::error::Error>> {
    e1009_unsafe_cell_misuse();
    Ok(())
}
