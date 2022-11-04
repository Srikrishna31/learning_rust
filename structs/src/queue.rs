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

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
}

