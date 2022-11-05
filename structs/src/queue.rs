pub struct Queue {
    pub older: Vec<char>,
    pub younger: Vec<char>
}

/// Functions defined in an impl block are called associated functions, since they're associated with
/// a specific type. The opposite of an associated function is a free function, one that is not
/// defined as an impl block's item.
impl Queue {
    pub fn push(&mut self, c:char) {
        self.younger.push(c);
    }

    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
        }

        //Bring the elements in younger over to older, and put them in the promised order.
        use std::mem::swap;
        swap(&mut self.older, &mut self.younger);
        self.older.reverse();

        self.older.pop()
    }

    ///If a method doesn't need to modify its self, then you can define it
    /// to take a shared reference instead.
    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    /// If a method wants to take ownership of self, it can take self by value.
    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.younger)
    }

    /// An impl block for a given type can also define functions that don't take self as an argument
    /// at all. These are still associated functions, since they're in an impl block, but they're
    /// not methods, since they don't take a self argument. These are called type-associated functions.
    pub fn new() -> Queue {
        Queue { older: Vec::new(), younger: Vec::new() }
    }
}

