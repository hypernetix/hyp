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
    pub fn e1009_bad_get_mut(&self) -> &mut T {
        // PROBLEM E1003: Direct use of unsafe code
        unsafe {
            // PROBLEM E1004: No safety documentation
            // DANGER: This creates aliasing mutable references
            &mut *self.value.get()
        }
    }
}

pub fn e1009_bad_unsafe_cell_misuse() {
    let cell = BadCell::new(42);

    // PROBLEM E1009: Two mutable references to the same memory!
    let ref1 = cell.e1009_bad_get_mut();
    let ref2 = cell.e1009_bad_get_mut();

    // Undefined behavior: modifying through both references
    *ref1 = 100;
    *ref2 = 200;

    // Which value is it? Undefined!
    println!("Value: {}", *ref1);
}

pub fn e1009_entry() -> Result<(), Box<dyn std::error::Error>> {
    e1009_bad_unsafe_cell_misuse();
    Ok(())
}

// ============================================================================
// GOOD EXAMPLES - Proper alternatives
// ============================================================================

use std::cell::{Cell, RefCell};
use std::sync::Mutex;

/// GOOD: Use Cell for simple Copy types
pub fn e1009_good_use_cell() {
    let cell = Cell::new(42);
    cell.set(100); // Safe interior mutability
    let value = cell.get();
    println!("Value: {}", value);
}

/// GOOD: Use RefCell for runtime borrow checking
pub fn e1009_good_use_refcell() {
    let cell = RefCell::new(42);

    // Borrow mutably - panics if already borrowed
    {
        let mut borrow = cell.borrow_mut();
        *borrow = 100;
    } // Mutable borrow ends here

    // Now we can borrow again
    let value = cell.borrow();
    println!("Value: {}", *value);
}

/// GOOD: Use Mutex for thread-safe interior mutability
pub fn e1009_good_use_mutex() {
    let mutex = Mutex::new(42);

    {
        let mut guard = mutex.lock().unwrap();
        *guard = 100;
    } // Lock released here

    let value = mutex.lock().unwrap();
    println!("Value: {}", *value);
}

/// GOOD: Use try_borrow to handle conflicts
pub fn e1009_good_try_borrow() {
    let cell = RefCell::new(42);

    let borrow1 = cell.borrow_mut();
    match cell.try_borrow_mut() {
        Ok(_) => println!("Got second borrow"),
        Err(_) => println!("Already borrowed - handled gracefully"),
    }
    drop(borrow1);
}

// ============================================================================
// GOOD EXAMPLES unit tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn e1009_good_use_cell_allows_updates() {
    e1009_good_use_cell();
    }

    #[test]
    fn e1009_good_use_refcell_borrows_mutably() {
    e1009_good_use_refcell();
    }

    #[test]
    fn e1009_good_use_mutex_updates_value() {
    e1009_good_use_mutex();
    }

    #[test]
    fn e1009_good_try_borrow_handles_conflict() {
        e1009_good_try_borrow();
    }
}
