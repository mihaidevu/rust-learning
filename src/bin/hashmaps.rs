/*
--- HashMap Characteristics (Rust) ---
* Each key appears only once (keys are unique)
* Inserting a value with an existing key replaces the old value
* The insert method returns the old value (if the key already existed)
* Stores data as key-value pairs
* Does not guarantee order of elements (unordered collection)
* Provides fast lookup, insertion, and deletion (average O(1))
* Keys must implement Eq and Hash traits
*/

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    let old = map.insert("a", 2);
    println!("Old value: {:?}", old);
    println!("Current value: {:?}", map.get("a"));
}

/* Output:
Old value: Some(1)
Current value: Some(2)
*/