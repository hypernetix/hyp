/// E1501: Non-Send types across threads
/// Severity: HIGH
/// LLM confusion: 4 (HIGH)
///
/// Description: Some types in Rust are not safe to send between threads (not "Send"). This example
/// shows code that compiles but demonstrates the confusion around thread safety. The code uses
/// channels and Arc to work around the Send requirement, but the pattern is confusing because it's
/// not immediately obvious why we need Arc instead of Rc. For non-Rust developers, understanding
/// which types are Send and which aren't is very confusing. Fix by using thread-safe types (Arc,
/// Mutex) from the start and understanding the Send/Sync traits.
///
/// Mitigation: Use `Arc` instead of `Rc` for multi-threaded code. The compiler will prevent this
/// error at compile time. Understand the Send and Sync traits - they mark types safe for threading.
/// Use thread-safe alternatives: Arc instead of Rc, Mutex/RwLock for shared mutable state.
use std::sync::{mpsc, Arc};
use std::thread;

pub fn e1501_non_send_across_threads() {
    // PROBLEM E1501: This pattern is confusing - why do we need Arc here?
    // Non-Rust developers won't understand the Send requirement
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let (tx, rx) = mpsc::channel();

    // Spawn multiple threads that share the data
    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let tx_clone = tx.clone();

        thread::spawn(move || {
            // PROBLEM E1501: The need for Arc instead of Rc is not obvious
            // LLMs often suggest Rc here, which won't compile
            let sum: i32 = data_clone.iter().sum();
            // PROBLEM E1002: direct unwrap/expect
            tx_clone.send((i, sum)).unwrap();
        });
    }

    drop(tx);

    // Collect results
    for (thread_id, sum) in rx {
        let _ = (thread_id, sum);
    }
}

// PROBLEM E1501: This struct is NOT Send because it contains Rc
// But this isn't obvious without understanding Send trait
use std::rc::Rc;

pub struct NotSendData {
    // PROBLEM E1501: Rc is not Send - can't be sent across threads
    // But this compiles fine as long as we don't try to send it
    data: Rc<Vec<i32>>,
}

impl NotSendData {
    pub fn new(data: Vec<i32>) -> Self {
        NotSendData {
            data: Rc::new(data),
        }
    }

    pub fn get_sum(&self) -> i32 {
        self.data.iter().sum()
    }
}

// This function compiles but demonstrates the confusion
pub fn e1501_confusing_send_requirement() {
    let not_send = NotSendData::new(vec![1, 2, 3]);

    // PROBLEM E1501: Can't spawn thread with not_send because it's not Send
    // We have to work around it, which is confusing
    let sum = not_send.get_sum();

    // Instead of sending the struct, we send just the data
    thread::spawn(move || {
        println!("Sum: {}", sum);
    });
}

pub fn e1501_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
