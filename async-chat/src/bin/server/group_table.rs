use crate::group::Group;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct GroupTable(Mutex<HashMap<Arc<String>, Arc<Group>>>);

/// A `GroupTable` is simply a mutex-protected hash table, mapping chat group names to actual groups,
/// both managed using reference-counted pointers. The `get` and `get_or_crate` methods lock the
/// mutex, perform a few hash table operations, perhaps some allocations, and return.
///
/// In `GroupTable`, we use a plain old `std::sync::Mutex`. There is no asynchronous code in this
/// module at all, so there are no `awaits` to avoid.
impl GroupTable {
    pub fn new() -> GroupTable {
        GroupTable(Mutex::new(HashMap::new()))
    }

    pub fn get(&self, name: &String) -> Option<Arc<Group>> {
        self.0.lock().unwrap().get(name).cloned()
    }

    pub fn get_or_create(&self, name: Arc<String>) -> Arc<Group> {
        self.0.lock().unwrap().entry(name.clone()).or_insert_with(|| Arc::new(Group::new(name)))
            .clone()
    }
}
