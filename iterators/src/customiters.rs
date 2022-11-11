/// You can implement the IntoIterator and Iterator traits for your own types, making all the adapters
/// and consumers available for use, along with lots of other library and crate code written to work
/// with the standard iterator interface.
pub(crate) struct I32Range {
    pub(crate) start: i32,
    pub(crate) end: i32,
}

impl Iterator for I32Range {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.start >= self.end {
            return None;
        }
        let result = Some(self.start);
        self.start +=1;
        result
    }
}
use enums_and_patterns::enums::{BinaryTree::*, BinaryTree, TreeNode};

/// The classic way to walk a binary tree is to recurse, using the stack of function calls to keep
/// track of your place in the tree and the nodes yet to be visited. But when implementing Iterator
/// for BinaryTree<T>, each call to next must produce exactly one value and return. To keep track of
/// the tree nodes it has yet to produce, the iterator must maintain its own stack.
pub(crate) struct TreeIter<'a, T> {
    /// A stack of references to tree nodes. Since we use `Vec`'s `push` and `pop` methods, the top
    /// of the stack is the end of the vector.
    ///
    /// The node the iterator will visit next is at the top of the stack, with those ancestors still
    /// unvisited below it. If the stack is empty, the iteration is over.
    unvisited: Vec<&'a TreeNode<T>>
}

impl<'a, T: 'a> TreeIter<'a, T> {
    fn push_left_edge(&mut self, mut tree: &'a BinaryTree<T>) {
        while let NonEmpty(ref node) = *tree {
            self.unvisited.push(node);
            tree = &node.left;
        }
    }
}
pub(crate) struct BinaryTreeExt<T>(pub(crate) BinaryTree<T>);

impl <T> BinaryTreeExt<T> {
    pub(crate) fn iter(&self) -> TreeIter<T> {
        let mut iter = TreeIter{unvisited: Vec::new()};
        iter.push_left_edge(&self.0);
        iter
    }
}

impl<'a, T: 'a> IntoIterator for &'a BinaryTreeExt<T> {
    type Item = &'a T;
    type IntoIter = TreeIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl <'a, T> Iterator for TreeIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        // Find the node this iteration must produce, or finish the iteration.
        let node = self.unvisited.pop()?;

        // After `node`, the next thing we produce must be the leftmost child in `node`'s right
        // subtree, so push the path from here down. Our helper method turns out to be just what we
        // need.
        self.push_left_edge(&node.right);

        Some(&node.element)
    }
}
