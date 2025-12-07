/// E1206: Deeply nested generic types
/// Severity: HIGH
/// LLM confusion: 4 (HIGH)
///
/// Description: This code has generic types nested within other generic types, creating a complex
/// type hierarchy that's extremely difficult to understand. Each layer of nesting adds cognitive
/// load - you have to track what each type parameter means at each level. It's like having
/// containers within containers within containers, where each container has its own type rules.
/// Fix by flattening the type hierarchy, using type aliases for complex nested types, or
/// restructuring to avoid deep nesting.
///
/// Mitigation: Use type aliases to name complex nested types: `type MyType<T> = Vec<Option<Result<T, E>>>`.
/// Limit generic nesting to 2-3 levels. Consider if the complexity is necessary - often simpler
/// designs work better. Use `#![warn(clippy::type_complexity)]` to catch overly complex types.
use std::collections::HashMap;

// This type alias shows 5 levels of generic nesting:
// 1. HashMap - maps String keys to values
// 2. Vec - each value is a vector of...
// 3. Option - which might contain...
// 4. Result - which is either Ok (success) or Err (error) containing...
// 5. Box<dyn Fn...> - a heap-allocated function that takes T and returns Result<Vec<T>, E>
// Reading this type requires mentally unwrapping 5 layers to understand what data it holds.
//
// PROBLEM E1206: Deeply nested generic types (5 levels)
pub type E1206ComplexType<T, E> =
    HashMap<String, Vec<Option<Result<Box<dyn Fn(T) -> Result<Vec<T>, E> + Send>, E>>>>;

// This function returns the deeply nested type defined above.
// The where clause ensures T can be cloned and sent between threads,
// and E is an error type that can also be sent between threads.
//
// PROBLEM E1206: Working with deeply nested types is confusing
pub fn e1206_deeply_nested_generics<T, E>() -> E1206ComplexType<T, E>
where
    T: Clone + Send + 'static,
    E: std::error::Error + Send + 'static,
{
    let mut map: HashMap<
        String,
        Vec<Option<Result<Box<dyn Fn(T) -> Result<Vec<T>, E> + Send>, E>>>,
    > = HashMap::new();

    map
}

// These three structs show nested generic structures:
// Container holds a Vec of Wrappers
// Wrapper holds a Box containing a Holder
// Holder holds an Option<Result<T>> - the actual data is buried 3 layers deep
// PROBLEM E1206: Nested generic structs
pub struct Container<T> {
    inner: Vec<Wrapper<T>>,
}

pub struct Wrapper<T> {
    data: Box<Holder<T>>,
}

pub struct Holder<T> {
    value: Option<Result<T, String>>,
}

// This function returns Container<Container<Container<T>>> - nesting the Container type 3 times!
// To access the inner T value, you'd need to unwrap 3 Container layers,
// plus the Vec, Box, Option, and Result inside each layer.
// PROBLEM E1206: Three levels of the same generic struct nested
pub fn e1206_nested_generic_structs<T: Clone>() -> Container<Container<Container<T>>> {
    unimplemented!("Deeply nested generic structs")
}

pub fn e1206_entry() -> Result<(), Box<dyn std::error::Error>> {
    let _: E1206ComplexType<i32, std::io::Error> = e1206_deeply_nested_generics();
    Ok(())
}
