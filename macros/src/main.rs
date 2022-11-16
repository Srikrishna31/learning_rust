#![recursion_limit="256"]

/// The compiler imposes a recursion limit on macros: 64 calls, by default. You can adjust it by
/// adding the below attribute at the top of the crate where the macro is used.
#[macro_use] mod json;
use json::Json;


fn main() {
    println!("Hello, world!");

    let v = my_vec![1,2,3];

    println!("{:?}", v);

    let students_macro = json!([
        {
            "name" : "Jim Blandy",
            "class_of": 1926,
            "major": "Tibetan throat singing"
        },
        {
            "name": "Jason Orendorff",
            "class_of": 1702,
            "major": "Knots"
        }
    ]);

    let students_hand_coded = Json::Array(vec![
        Json::Object(Box::new(vec![
            ("name".to_string(), Json::String("Jim Blandy".to_string())),
            ("class_of".to_string(), Json::Number(1926.0)),
            ("major".to_string(), Json::String("Tibetan throat singing".to_string()))
        ].into_iter().collect())),
        Json::Object(Box::new(vec![
            ("name".to_string(), Json::String("Jason Orendorff".to_string())),
            ("class_of".to_string(), Json::Number(1702.0)),
            ("major".to_string(), Json::String("Knots".to_string()))
        ].into_iter().collect()))
    ]);

    assert_eq!(students_macro, students_hand_coded);
}
