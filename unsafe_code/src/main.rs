
use crate::my_ascii::Ascii;
use crate::ref_with_flag::RefWithFlag;

mod my_ascii;
mod ref_with_flag;
mod gap_buffer;


/// A union representing a collection of bytes that can be interpreted as either an integer or a
/// floating-point number would be written as follows:
/// This is a union with two fields, f and i. They can be assigned to just like the fields of a struct,
/// but when constructing a union, unlike struct, you must choose exactly one. Where the fields of α
/// struct refer to different positions in memory, the fields of a union refer to different interpretations
/// of the same sequence of bits. Assigning to a different field simply means overwriting some or all
/// of those bits, in accordance with an appropriate type.
union FloatOrInt {
    f: f32,
    i: i32,
}

/// The size of a union is determined by its largest field.
union SmallOrLarge {
    s: bool,
    l: u64,
}

fn main() {
    let bytes: Vec<u8> = b"ASCII and ye shall receive".to_vec();

    //This call entails no allocation or text copies, just a scan.
    let ascii = my_ascii::Ascii::from_bytes(bytes).unwrap();

    let string = String::from(ascii);

    assert_eq!(string, "ASCII and ye shall receive");

    // Imagine that this vector is the result of some complicated process that we expected to produce
    // ASCII. Something went wrong!
    let bytes = vec![0xf7, 0xbf, 0xbf, 0xbf];

    let ascii = unsafe {
        // This unsafe function's contract is violated when `bytes` holds non-ASCII bytes.
        Ascii::from_bytes_unchecked(bytes)
    };

    let bogus: String = ascii.into();

    // `bogus` now holds ill-formed UTF-8. Parsing its first character produces a `char` that is not
    // a valid unicode code point. That's undefined behavior, so the language doesn't say how this
    // assertion should behave.
    assert_eq!(bogus.chars().next().unwrap() as u32, 0x1ff_fff);

    let v: Vec<usize> = my_ascii::zeroed_vector(100_000);

    assert!(v.iter().all(|&u| u == 0));

    ref_with_flag::raw_pointers();

    let vec = vec![10,20,30];
    let flagged = RefWithFlag::new(&vec, true);
    assert_eq!(flagged.get_ref()[1], 20);
    assert_eq!(flagged.get_flag(), true);

    assert_eq!(std::mem::size_of::<i64>(), 8);
    assert_eq!(std::mem::align_of::<(i32, i32)>(), 4);

    //Fat pointers to slices carry their referent's length
    let slice: &[i32] = &[1,3,9,27,81];
    assert_eq!(std::mem::size_of_val(slice), 20);

    let text:&str = "alligator";
    assert_eq!(std::mem::size_of_val(text), 9);
    //assert_eq!(std::mem::size_of::<&str>(), 4);

    use std::fmt::Display;
    let unremarkable: &dyn Display = &193_u8;
    let remarkable: &dyn Display = &0.0072973525664;

    // These return the size/alignment of the value the trait object points to, not those of the trait
    // object itself. This information comes from the vtable the trait_object refers to.
    assert_eq!(std::mem::size_of_val(unremarkable), 1);
    assert_eq!(std::mem::align_of_val(remarkable), 8);

    let mut one = FloatOrInt{i: 1};
    assert_eq!(unsafe{one.i}, 0x00_00_00_01);
    one.f = 1.0;
    assert_eq!(unsafe{one.i}, 0x3F_80_00_00);

    let u = SmallOrLarge { l: 1337};
    println!("{}", unsafe{u.l}); // prints 1337

    let float = FloatOrInt { f: 31337.0};
    println!("{:b}", unsafe{float.i});

    assert_eq!(sign(-1), true);
    assert_eq!(sign(1), false);
    assert_eq!(sign(i64::MAX), false);
    assert_eq!(sign(i64::MIN), true);

    unsafe {
        match u {
            SmallOrLarge{s: true} => {println!("boolean true");}
            SmallOrLarge{l: 2} => {println!("integer 2");}
            _ => {println!("something else");}
        }
    }

    // A match arm that matches against a union variant without specifying a value will always succeed.
    // The following code will cause undefined behavior if the last written field of u was u.i
    unsafe {
        match float {
            FloatOrInt { f} => {println!("float {f}");}
            // warning: unreachable pattern
            FloatOrInt {i} => {println!("int {i}");}
        }
    }
}

/// Adding the attribute #[repr(C)] guarantees that all fields start at offset 0, rather than wherever
/// the compiler likes.
#[repr(C)]
union SignExtractor {
    /// Here the sign bit is the most significant bit of the most significant byte. Because x86 processors
    /// are little-endian, the order of those bytes is reversed; the most significant byte is not
    /// bytes[0], but bytes[7].
    value: i64,
    /// Because unions can't tell how to drop their contents, all their fields must be Copy.
    bytes: [u8; 8]
}

fn sign(int: i64) -> bool {
    let se = SignExtractor{value: int};

    println!("{:b} ({:?})", unsafe {se.value}, unsafe {se.bytes});
    unsafe { se.bytes[7] >= 0b10000000}
}
