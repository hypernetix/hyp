/// E1605: Rc cycle memory leak
/// Severity: MED
/// LLM confusion: 3 (MED)
///
/// Description: Reference cycles with Rc (reference counting) cause memory leaks because the
/// reference count never reaches zero. Node1 holds a reference to Node2, and Node2 holds a
/// reference to Node1, so both have count >= 1 forever. Fix by using Weak references to break
/// cycles, or use a different data structure that doesn't create cycles.
///
/// Mitigation: Use `Weak` references to break cycles - Weak doesn't increment the reference count.
/// Design data structures to avoid cycles when possible (e.g., parent owns children, children have
/// Weak references to parent). Use arena allocators for graph-like structures.

pub fn e1605_rc_cycle() {
    use std::cell::RefCell;
    use std::rc::Rc;

    struct Node {
        next: Option<Rc<RefCell<Node>>>,
    }

    let node1 = Rc::new(RefCell::new(Node { next: None }));
    let node2 = Rc::new(RefCell::new(Node {
        next: Some(node1.clone()),
    }));

    // PROBLEM E1605: Creating reference cycle, memory will leak
    node1.borrow_mut().next = Some(node2.clone());
}

pub fn e1605_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
