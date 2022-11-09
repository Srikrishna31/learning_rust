use crate::closure::has_monster_attacks;

mod closure;

fn main() {
    let cities = vec![closure::City{
        name: "Delhi".to_string(),
        population: 1_000_000,
        country:"India".to_string(),
        monster_attack_risk: 0.05
    },
    closure::City{
        name: "Bangalore".to_string(),
        population: 850_000,
        country: "India".to_string(),
        monster_attack_risk: 0.03
    },
    closure::City {
        name: "Chennai".to_string(),
        population: 950_000,
        country: "India".to_string(),
        monster_attack_risk: 0.06
    }];

    let n = closure::count_selected_cities(&cities, has_monster_attacks);
    println!("{n}");

    let limit = 0.04;

    let n = closure::count_selected_cities_general(&cities, |city| city.monster_attack_risk > limit);
    println!("{n}");

    let mut i = 0;
    let incr = || {
        i += 1;
        println!("Ding! i is now {i}");
    };

    closure::call_twice(incr);

    assert_eq!(i, 2);

    let y = 10;
    let add_y = |x| x + y;
    let copy_of_add_y = add_y;  //This closure is `Copy`

    assert_eq!(add_y(copy_of_add_y(22)), 42);

    let mut x = 0;
    let mut add_to_x = |n:i32| { x += n; x};
    let copy_of_add_to_x = add_to_x;    //this moves, rather than copy.
    //assert_eq!(add_to_x(copy_of_add_to_x(1)), 2); //error: use of moved value.

    let mut greeting = String::from("Hello, ");
    let greet = move |name| {
        greeting.push_str(name);
        println!("{greeting}");
    };
    greet.clone()("Alfred");
    greet.clone()("Bruce");
}
