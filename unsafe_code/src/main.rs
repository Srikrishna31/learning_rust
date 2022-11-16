use crate::my_ascii::Ascii;

mod my_ascii;
mod ref_with_flag;

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
}
