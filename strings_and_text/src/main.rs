use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    assert!(32u8.is_ascii_whitespace());
    assert!(b'9'.is_ascii_digit());

    let line_tab = '\u{000b}';      // 'line tab' AKA 'vertical tab'
    assert_eq!(line_tab.is_whitespace(), true);
    assert_eq!(line_tab.is_ascii_whitespace(), false);

    assert_eq!('F'.to_digit(16), Some(15));
    assert_eq!(std::char::from_digit(15,16), Some('f'));
    assert!(char::is_digit('f', 16));

    // The uppercase form of the German letter "sharp S" is "SS"
    // let mut upper = 'β'.to_uppercase();
    //
    // println!("{upper}");
    // assert_eq!(upper.next(), Some('S'));
    // assert_eq!(upper.next(), Some('S'));
    // assert_eq!(upper.next(), None);

    collect();

    range();

    extend();

    format()?;

    add();

    drain();

    replace_range();

    search_patterns();

    find_replace();

    println!("Hello, world!");

    Ok(())
}

/// Rust's String and str types are guaranteed to hold only well-formed UTF-8. The library ensures this
/// by restricting the ways you can create String and str values and the operations you can perform on
/// them, such that the values are well-formed when introduced and remain so as you work with them.
/// All their methods protect this guarantee: no safe operation on them can introduce ill-formed UTF-8.
/// This simplifies the code that works on text.
fn collect() {
    let spacey = "man hat tan";
    let spaceless:String = spacey.chars().filter(|c| !c.is_whitespace()).collect();

    assert_eq!(spaceless, "manhattan");
}

/// slice range
/// Note that you cannot index a string slice with a single position, like slice[i]. Fetching a
/// single character at a given byte offset is a bit clumsy: you must produce a chars iterator over
/// the slice, and ask it to parse one character's UTF-8.
fn range() {
    let full = "bookkeeping";
    assert_eq!(&full[..4], "book");
    assert_eq!(&full[5..], "eeping");
    assert_eq!(&full[2..4], "ok");
    assert_eq!(full[..].len(), 11);
    assert_eq!(full[5..].contains("boo"), false);
}

fn extend() {
    let mut also_spaceless = "con".to_string();
    also_spaceless.extend("tri but ion".split_whitespace());
    assert_eq!(also_spaceless, "contribution");
}

use std::fmt::Write;

/// String implements std::fmt::Write, meaning that the write! and writeln! macros can append formatted
/// text to Strings
fn format() -> Result<(), Box<dyn Error>>{
    let mut letter = String::new();
    writeln!(letter, "Whose {} these are I think I know", "rutabagas")?;
    writeln!(letter, "His house is in the village though;")?;
    assert_eq!(letter, "Whose rutabagas these are I think I know\n\
                        His house is in the village though;\n");

    Ok(())
}

/// Since String implements Add<&str> and AddAssign<&str>, you can write code like this
fn add() {
    let left = "partners".to_string();
    let mut right = "crime".to_string();
    assert_eq!(left + " in " + &right, "partners in crime");

    right += " doesn't pay";
    assert_eq!(right, "crime doesn't pay");
}

/// Drain: Returns an iterator over the given range of byte indices and removes the characters once
/// the iterator is dropped. Characters after the range are shifted toward the front
fn drain() {
    let mut choco = "chocolate".to_string();
    assert_eq!(choco.drain(3..6).collect::<String>(), "col");
    assert_eq!(choco, "choate");

    // If you just want to remove the range, you can just drop the iterator immediately, without
    // drawing any items from it.
    let mut winston = "Churchill".to_string();
    winston.drain(2..6);
    assert_eq!(winston, "Chill");
}

fn replace_range() {
    let mut beverage = "a pina colada".to_string();
    beverage.replace_range(2..7, "kahlua");
    assert_eq!(beverage, "a kahluacolada");
}

/// The standard library supports four main kinds of patterns:
/// * A char as a pattern matches that character.
/// * A String or &str or &&str as a pattern matches a substring equal to the pattern.
/// * A FnMut(char) -> bool closure as a pattern matches a single character for which the closure
/// returns true.
/// * A &[char] as a pattern(not a &str, but a slice of char values) matches any single character that
/// appears in the list. Note that if you write out the list as an array literal, you may need to call
/// as_ref() to get the type right.
fn search_patterns() {
    let haystack = "One fine day, in the middle of the night";

    assert_eq!(haystack.find(','), Some(12));
    assert_eq!(haystack.find("night"), Some(35));
    assert_eq!(haystack.find(char::is_whitespace), Some(3));

    assert_eq!("## Elephants".trim_start_matches(|ch: char| ch == '#' || ch.is_whitespace()),
            "Elephants");

    let code = "\t      function noodle() { ";

    assert_eq!(code.trim_start_matches([' ', '\t'].as_ref()),
                "function noodle() { ");
}

fn find_replace() {
    let quip = "We also know there are known unknowns";
    assert_eq!(quip.find("know"), Some(8));
    assert_eq!(quip.rfind("know"), Some(31));
    assert_eq!(quip.find("ya know"), None);
    assert_eq!(quip.rfind(char::is_uppercase), Some(0));

    assert_eq!("The only thing we have to fear is fear itself".replace("fear", "spin"),
            "The only thing we have to spin is spin itself");

    assert_eq!("`Borrow` and `BorrowMut`"
        .replace(|ch:char| !ch.is_alphanumeric(), ""), "BorrowandBorrowMut");

    /// Because the replacement is done eagerly, .repleace()'s behavior on overlapping matches can be
    /// surprising. Here, there are four instances of the pattern, "aba", but the second and fourth
    /// no longer match after the first and third are replaced.
    assert_eq!("cabababababbage"
        .replace("aba", "***"),
        "c***b***babbage");
}
