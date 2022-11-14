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

    split_terminator();

    split_whitespace();

    parse_string()?;

    println!("Greetings, {}!", get_name());

    format_value();

    regex();

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
use std::net::IpAddr;

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


/// Split_terminator
/// This is similar to split, except that the pattern is treated as a terminator, not a separator: if
/// pattern matches at the vey end of slice, the iterators do not produce an empty slice representing
/// the empty string between that match and the end of the slice as split and rsplit do.
fn split_terminator() {
    // The ':' characters are separators here. Note the final "".
    assert_eq!("jimb:1000:Jim Blandy:".split(':').collect::<Vec<_>>(),
                vec!["jimb", "1000", "Jim Blandy", ""]);

    // The '\n' characters are terminators here
    assert_eq!("127.0.0.1 localhost\n\
                127.0.0.1 www.reddit.com\n"
        .split_terminator('\n').collect::<Vec<_>>(),
        vec!["127.0.0.1 localhost",
            "127.0.0.1 www.reddit.com"]);
        // Note, no fineal ""!
}

/// split_whitespace
/// Return an iterator over the whitespace-separated portions of slice. A run of multiple whitespace
/// characters is considered a single separator. Trailing whitespace is ignored.
fn split_whitespace() {
    let poem = "This is just to say\n\
                      I have eaten\n\
                      the plums\n\
                      again\n";
    assert_eq!(poem.split_whitespace().collect::<Vec<_>>(),
                vec!["This", "is", "just", "to", "say", "I", "have", "eaten", "the"
                        , "plums", "again"]);
}


/// Parsing other types from Strings
/// Rust provides standard traits from both parsing values from strings and producing textual representation
/// of values.
/// If a type implements the std::std::FromStr trait, then it provides a standard way to parse a value
/// from a string slice.
///
///     pub trait FromStr: Sized {
///         type Err;
///         fn from_str(s: &str) -> Result<Self, Self::Err>
///     }
///
///
fn parse_string() -> Result<(), Box<dyn Error>>{
   use std::str::FromStr;

    assert_eq!(usize::from_str("3628800"), Ok(3628800));
    assert_eq!(f64::from_str("128.5625"), Ok(128.5625));
    assert_eq!(bool::from_str("true"), Ok(true));

    assert!(f64::from_str("not a float at all").is_err());
    assert!(bool::from_str("TRUE").is_err());

    use std::net::IpAddr;

    let address = IpAddr::from_str("fe80::0000:3ea9:f4ff:fe34:7a50")?;
    assert_eq!(address, IpAddr::from([0xfe80, 0,0,0,0x3ea9,0xf4ff,0xfe34,0x7a50]));

    Ok(())
}


use std::borrow::Cow;

fn get_name() -> Cow<'static, str> {
    std::env::var("USER")
        .map(|v| Cow::Owned(v))
        .unwrap_or(Cow::Borrowed("whoever you are"))
}

fn format_value() {
    use std::collections::HashMap;;

    let mut map = HashMap::new();
    map.insert("Portland", (45.5237606, -122.6819273));
    map.insert("Taipei", (25.0375167, 121.5637));

    //# tells Rust to do pretty printing.
    println!("{:#?}", map);

    use std::rc::Rc;

    let original = Rc::new("mazurka".to_string());
    let cloned = original.clone();
    let imposter = Rc::new("mazurka".to_string());
    println!("text: {original}, {cloned}, {imposter}");

    // The {:p} notation formats references, boxes and other pointer-like types as addresses
    println!("text: {original:p}, {cloned:p}, {imposter:p}");

}

use regex::Regex;

fn regex(){
    let semver = Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[-.[:alnum:]]*)?").unwrap();

    let haystack = r#"regex = "0.2.5""#;
    assert!(semver.is_match(haystack));

    let captures = semver.captures(haystack).ok_or("semver regex should have matched").unwrap();
    assert_eq!(&captures[0], "0.2.5");
    assert_eq!(&captures[1], "0");
    assert_eq!(&captures[2], "2");
    assert_eq!(&captures[3], "5");

    assert_eq!(captures.get(4), None);
    assert_eq!(captures.get(3).unwrap().start(), 13);
    assert_eq!(captures.get(3).unwrap().end(), 14);
    assert_eq!(captures.get(3).unwrap().as_str(), "5");

    let new_haystack = "In the beginning, there was 1.0.0. \
                              For a while we used 1.0.1-beta \
                              but in the end, we settled on 1.2.4.";

    let matches : Vec<&str> = semver.find_iter(new_haystack)
        .map(|match_| match_.as_str())
        .collect();

    assert_eq!(matches, vec!["1.0.0", "1.0.1-beta", "1.2.4"]);
}


fn regex_from_commandline(){
    use lazy_static::lazy_static;

    lazy_static! {
        static ref SEMVER: Regex =
            Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[-.[:alnum:]]*)?")
        .expect("error parsing regex");
    }

    use std::io::BufRead;

    let stdin = std::io::stdin();
    for line_result in stdin.lock().lines() {
        let line = line_result.unwrap();
        if let Some(match_) = SEMVER.find(&line) {

            println!("{}", match_.as_str());
        }
    }

}

/// Unicode has two ways to represent the accented text:
/// * The composed form, where the text is written with accented characters.
/// * The decomposed form, where the text is written in ascii, without accents and followed by code
/// points (for eg. '\u{301}', which is the COMBINING ACUTE ACCENT), that modify the character preceding
/// them.
/// Fortunately, Unicode specifies normalized forms for strings. Whenever two strings should be treated
/// as equivalent according to Unicode's rules, their normalized forms are character-for-character
/// identical. When encoded with UTF-8, they are byte-for-byte identical. This means you can compare
/// normalized strings with ==, use them as keys in a HashMap or HashSet, and so on, and you'll get
/// Unicode's notion of equality.
/// Taking a normalized string and normalizing it again in the same form is guaranteed to return identical
/// text.
/// Although any substring of a normalized string is itself normalized, the concatenation of two normalized
/// strings is not necessarily normalized: for example, the second string might start with combining
/// characters that should be placed before combining characters at the end of the first string.
/// As long as a text uses no unassigned code points when it is normalized, Unicode promises that its
/// normalized form will not change in future versions of the standard. This means that canonical forms
/// are generally safe to use in persistent storage, even as the Unicode standard evolves.
fn unicode_normalization() {
    assert!("th\u{e9}" != "the\u{301}");
    assert!("th\u{e9}" > "the\u{301}");
}
