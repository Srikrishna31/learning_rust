/// When a value's owner goes away, we say that Rust drops the value. Dropping a value entails
/// freeing whatever other values, heap storage, and system resources the value owns. Drops occur
/// under a variety of circumstances: when a variable goes out of scope; at the end of an expression
/// statement; when you truncate a vector, removing elements from its end; and so on. For the most
/// part, Rust handles dropping values for you automatically.
pub(crate) struct Appellation {
    pub(crate) name: String,
    pub(crate) nicknames: Vec<String>
}

/// If you want you can customize how Rust drops values of your type by implementing the std::ops::Drop trait:
///     trait Drop {
///         fn drop(&mut self);
///     }
/// An implementation of Drop is analogous to a destructor in C++, or a finalizer in other languages.
/// When a value is dropped, if it implements std::ops::Drop, Rust calls its drop method, before
/// proceeding to drop whatever values its fields or elements own, as it normally would. This implicit
/// invocation of drop is the only way to call that method; if you try to invoke it explicitly yourself,
/// Rust flags that as an error.
/// Because Rust calls Drop::drop on a value before dropping its fields or elements, the value the
/// method receives is always still fully initialized.
impl Drop for Appellation {
    fn drop (&mut self) {
        print!("Dropping {}", self.name);
        if !self.nicknames.is_empty() {
            print!(" (AKA {})", self.nicknames.join(", "));
        }
        println!();
    }
}
