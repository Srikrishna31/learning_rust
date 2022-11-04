use std::ops::Range;

/// Return true if two ranges overlap.
///
///     assert_eq!(ranges::overlap(0..7, 3..10), true);
///     assert_eq!(ranges::overlap(1..5, 101..105), false);
///
/// If either range is empty, they don't count as overlapping.
///
///     assert_eq!(ranges::overlap(0..0, 0..10), false);
pub fn overlap(r1: Range<usize>, r2: Range<usize>) -> bool {
    r1.start < r1.end && r2.start < r2.end && r1.start < r2.end && r2.start < r1.end
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    fn roughly_equal(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

    #[test]
    fn trig_works() {
        use std::f64::consts::PI;
        assert!(roughly_equal(PI.sin(), 0.0));
    }
}


/// Whereas crates are about code sharing between projects, modules are about code organization within a
/// project. They act as Rust's namespaces, containers for the functions, types, constants, and so on
/// that make up your Rust program or library.
///
/// A module is a collection of items, named features like structs, functions. The 'pub' keyword makes
/// an item public, so it can be accessed from outside the module.
///
/// Any item marked 'pub(crate)' means that it is available anywhere inside this crate, but isn't exposed
/// as part of the external interface. It can't be used by other crates, and it won't show up in this
/// crate's documentation.
///
/// Anything that isn't marked pub is private and can only be used in the same module in which it is
/// defined, or any child modules. Marking an item pub is often known as exporting that item.
///
/// Modules can nest, and it's fairly common to see a module that's just a collection of submodules.
/// If you want an item in a nested module to be visible to other crates, be sure to make it an all
/// enclosing modules as public.
///
/// It's also possible to specify pub(super), making an item visible to the parent module only, and in
/// pup(in <path>), which makes it visible in a specific parent module and its descendants.
///     mod plant_structures {
///         pub mod roots {
///             pub mod products {
///                 pub(in crate::plant_structures::root) struct Cytokinin {
///                     ...
///                 }
///                 use products::Cytokinin;    //ok: in `roots` module.
///         }
///         use roots::products::Cytokinin;     //error: `Cytokinin` is private
///     }
///
/// Modules in Separate files or in their own directories
/// A module can be written like this:
///     mod spores;
/// When Rust sees mod spores;', it checks for both spores.rs and spores/mod.rs; if neither file
/// exists or both exist, that's an error. It's also possible to use a file and directory with the
/// same name to make up a module.
///
/// These three options - modules in their own file, modules in their own directory with a mod.rs,
/// and modules in their own file with a supplementary directory containing submodules - give the
/// module system enough flexibility to support almost any project structure you might desire.
///
/// Paths and Imports
///     use std::mem;
///
/// The use declaration causes the name mem to be a local alias for std::mem throughout the enclosing
/// block or module.
///
/// Modules donot automatically inherit names from their parent modules. Instead, each module starts
/// with a blank slate and must import the names it uses.
/// The keywords `super` and `crate` have a special meaning in paths: super refers to the parent
/// module, and crate refers to the crate containing the current module.
/// Using paths relative to the crate root rather than the current module makes it easier to move
/// code around the project, since all the imports won't break if the path of the current module
/// changes.
/// Submodules can access private items in their parent modules with `use super::*`.
///
/// Modules aren't the same thing as files, but there is a natural analogy between modules and the
/// files and directories of a Unix filesystem. The use keyword creates aliases, just as the ln command
/// creates links. Paths, like filenames, come in absolute and relative forms. self and super are
/// like the . and .. special directories.
///
///
/// A struct's fields, even private fields are accessible throughout the module where the struct is
/// declared, and its submodules. Outside the module, only public fields are accessible.
///
/// Statics and Constants
/// The `const` keyword introduces a constant. The `static` keyword introduces a static item.
///
/// A constant is a bit like a C++ #define: the value is compiled into your code every place it's
/// used. A static is a variable that's setup before your program starts running and lasts until it
/// exits. Use constants for magic numbers and strings in your code. Use statics for larger amounts
/// of data, or anytime you need to borrow a reference to the constant value.
///
/// There are mut constants. Statics can be marked mut, but Rust has no way to enforce its rules
/// about exclusive access on mut statics. They are, therefore inherently non-thread safe, and safe
/// code can't use them at all. Rust discourages global mutable state.
///
///
/// Versions
/// The version compatibility rules are adapted from Semantic versioning [https://semver.org]
/// * A version number that starts with 0.0 is so raw that Cargo never assumes it's compatible with
/// any other version.
/// * A version number that starts with 0.x, where x is nonzero, is considered compatible with other
///point releases in the 0.x series.
/// * Once a project reaches 1.0, only new major versions break compatibility. So if you ask for
/// version 2.0.1, Cargo might use 2.17.99 instead, but not 3.0
///
///
///
/// Cargo.lock
/// A Cargo.lock file records the exact version of every crate used in the project. Later builds
/// will consult this file and continue to use the same versions. Cargo upgrades to newer versions
/// only when you tell it to, either by manually bumping up the version number in your Cargo.toml
/// file or by running cargo update.
///
/// Cargo.lock is automatically generated for you, and you normally won't edit it by hand. Nonetheless,
/// if your project is an executable, you should commit Cargo.lock to version control. That way,
/// everyone who builds your project will consistently get the same versions. The history of your
/// Cargo.lock file will record your dependency updates.
///
/// If your project is an ordinary Rust library, don't bother committing Cargo.lock. Your library's
/// downstream users will have Cargo.lock files that contain version information for their entire
/// dependency graph; they will ignore your library's Cargo.lock file. In the rare case that your
/// project is a shared library (.dll, .dylib or .so file), there is no such downstream cargo user,
/// and you should therefore commit Cargo.lock.
///
/// Cargo.toml's flexible version specifiers make it easy to use Rust libraries in your project and
/// maximize compatibility among libraries. Cargo.lock's bookkeeping supports consistent, reproducible
/// builds across machines.

