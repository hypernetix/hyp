/// E1509: Channel lifetime issues
/// Severity: MED
/// LLM confusion: 3 (MED)
///
/// Description: Channels have a sender and receiver. If the sender is dropped before the receiver
/// tries to receive, the receiver gets a disconnection error. This code spawns a thread that
/// sends a message and immediately exits (dropping the sender), which might happen before the
/// receiver tries to receive. Fix by keeping the sender alive until all messages are sent, or
/// handle disconnection errors properly.
///
/// Mitigation: Keep senders alive until all messages are sent. Handle `RecvError` properly - it
/// indicates the sender was dropped. Use `sync_channel` for backpressure. Clone senders if
/// multiple threads need to send. Join threads before receiving if order matters.

pub fn e1509_channel_lifetime() {
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        // PROBLEM E1002: direct unwrap/expect
        // PROBLEM E1509: Sender dropped too early, receiver will get disconnected error
        tx.send(42).unwrap();
    });

    // May fail if sender dropped before this runs
    let _value = rx.recv();
}

pub fn e1509_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
