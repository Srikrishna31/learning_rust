mod queue;
///Rust has three kinds of struct types, named-field, tuple-like and unit-like, which differ in how
/// you refer to their components: a named-field struct gives a name to each component, whereas a
/// tuple-like struct identifies them by the order in which they appear. Unit-like structs have no
/// components at all.

///Like all other items, structs are private by default, visible only in the module where they're
/// declared and its submodules. You can make a struct visible outside its module by prefixing its
/// definition with pub. The same goes for each of its fields, which are also private by default.
pub struct GrayscaleMap {
    pub pixels: Vec<u8>,
    size: (usize, usize)
}

fn main() {

    let width = 1024;
    let height = 576;
    let image = GrayscaleMap {
        pixels: vec![0; width*height],
        size: (width, height)
    };
    println!("Hello, world!");

    assert_eq!(image.size, (1024, 576));
    assert_eq!(image.pixels.len(), 1024*576);

    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater
    };

    let (hokey1, hokey2) = chop(hokey);
    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey1.height, 30);
    assert_eq!(hokey1.health, 100);

    assert_eq!(hokey2.name, "Hokey II");
    assert_eq!(hokey2.height, 30);
    assert_eq!(hokey2.health, 100);

    //Construct an object of tuple like struct
    let image_bounds = Bounds(1024, 768);

    let mut q = queue::Queue{older: Vec::new(), younger: Vec::new() };

    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));

    q.push('=');
    assert_eq!(q.pop(), Some('1'));
    assert_eq!(q.pop(), Some('='));
    assert_eq!(q.pop(), None);
}

///
/// A struct expression starts with the type name and lists the name and value of each field, all
/// enclosed in curly braces. There's also shorthand for populating fields from local variables or
/// arguments with the same name:
fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap {
    assert_eq!(pixels.len(), size.0*size.1);
    GrayscaleMap{pixels, size}
}


struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent
}

#[derive(Copy, Clone)]
enum BroomIntent { FetchWater, DumpWater }

/// In a struct expression, if the named fields are followed by .. EXPR, then any fields not
/// mentioned take their values from EXPR, which must be another value of the same struct type.
//Receive the input Broom by value, taking ownership
fn chop(b: Broom) -> (Broom, Broom) {
    //Initialize `broom1` mostly from `b`, changing only `height`.
    let mut broom1 = Broom {height: b.height / 2, .. b};

    let mut broom2 = Broom{name: broom1.name.clone(), .. broom1};

    broom1.name.push_str(" I");
    broom2.name.push_str(" II");

    (broom1, broom2)
}


/// The second kind of struct type is called a tuple-like struct, because it resembles a tuple:
struct Bounds(usize, usize);

/// The values held by a tuple-like struct are called elements, just as the values of a tuple are.
/// You access them just as you would a tuple's:
///     assert_eq!(image_bounds.0*image_bounds.1, 786432);
/// Tuple like structs are good fit for pattern matching. They are also good for newtypes, structs
/// with a single component that you define to get stricter type checking.
///
/// Unit-like Structs
/// The third kind of struct is a little obscure: it declares a struct type with no elements at all:
struct Onesuch;

/// A value of such a type occupies no memory, much like the unit type (). Rust doesn't bother actually
/// storing unit-like struct values in memory or generating code to operate on them, because it can
/// tell everything it might need to know about the value from its type alone. But logically, an empty
/// struct is a type with values like any other - or more precisely, a type of which there is a
/// single value.
struct Dummy;
