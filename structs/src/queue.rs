pub struct Queue<T> {
    pub older: Vec<T>,
    pub younger: Vec<T>
}

/// Functions defined in an impl block are called associated functions, since they're associated with
/// a specific type. The opposite of an associated function is a free function, one that is not
/// defined as an impl block's item.
impl<T> Queue<T> {
    pub fn push(&mut self, c:T) {
        self.younger.push(c);
    }

    pub fn pop(&mut self) -> Option<T> {
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
    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.younger)
    }

    /// An impl block for a given type can also define functions that don't take self as an argument
    /// at all. These are still associated functions, since they're in an impl block, but they're
    /// not methods, since they don't take a self argument. These are called type-associated functions.
    pub fn new() -> Queue<T> {
        Queue { older: Vec::new(), younger: Vec::new() }
    }
}

/// This impl block header reads, Here are some associated functions specifically for Queue<f64>.
/// This gives Queue<f64> a sum method, available on no other kind of Queue.
impl Queue<f64> {
    pub fn sum(&self) -> f64 {
        0.0
    }
}


pub(in crate) struct Extrema<'elt> {
    pub greatest: &'elt i32,
    pub least: &'elt i32
}

pub (crate) fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
    let mut greatest = &slice[0];
    let mut least = &slice[0];

    for i in 1..slice.len() {
        if slice[i] < *least { least = &slice[i];}
        if slice[i] > *greatest {greatest = &slice[i];}
    }

    Extrema {greatest, least}
}
