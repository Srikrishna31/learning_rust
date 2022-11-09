
pub(crate) struct City {
    pub(crate) name: String,
    pub(crate) population: i64,
    pub(crate) country: String,
    pub(crate) monster_attack_risk: f32,
}

pub(crate) struct Statistic {
    avg: f32,
    std_dev: f32,
    var: f32,
}

impl City {
    fn get_statistic(&self, stat: &Statistic) -> i64 {
        self.population
    }
}

fn city_population_descending(city: &City) -> i64 {
    -city.population
}

fn sort_cities(cities: &mut Vec<City>) {
    cities.sort_by_key(city_population_descending);
}

/// A closure is an anonymous function expression.
fn sort_cities_with_closure(cities: &mut Vec<City>) {
    cities.sort_by_key(|city| -city.population);
}


/// Closures that borrow
/// A closure can use data that belongs to an enclosing function.
/// In the below case, when Rust creates the closure, it automatically borrows a reference to stat.
/// It stands to reason: the closure refers to stat, so it must have a reference to it. Since the
/// closure contains a reference to stat, Rust won't let it outlive stat.
fn sort_by_statistic(cities: &mut Vec<City>, stat: Statistic) {
    cities.sort_by_key(|city| -city.get_statistic(&stat));
}

use std::thread;
/// Closures that steal
/// The move keyword tells Rust that a closure doesn't borrow the variables it uses: it steals them.
/// * Just as everywhere else in the language, if a closure would move a value of a copyable type,
/// like i32, it copies the value instead.
/// * Values of noncopyable types, like Vec<City>, really are moved: the following code transfers
/// cities to new thread, by way of the move closure. Rust would not let us access cities by name after
/// creating the closure.
/// * If we needed to use the 'cities' after the point where the closure moves it, we could tell Rust
/// to clone cities and store the copy in a different variable. The closure would only steal one of
/// the copies - whichever one it refers to.
fn start_sorting_thread(mut cities: Vec<City>, stat: Statistic) -> thread::JoinHandle<Vec<City>>
{
    let key_fn = move |city: &City| -> i64 { -city.get_statistic(&stat)};

    thread::spawn(move || {
        cities.sort_by_key(key_fn);
        cities
    })
}

/// Function and Closure Types
/// You can do all the same things with functions that you do with other values. You can store them
/// in variables. You can use all the usual Rust syntax to compute function values.
/// Structs may have function-typed fields. Generic types like Vec can store scads of functions, as
/// long as they all share the same fn type. And function values are tiny: a fn value is the memory
/// address of the function's machine code, just like a function pointer in C++.
/// A function can take another function as an argument.
pub(crate) fn count_selected_cities(cities: &Vec<City>, test_fn: fn(&City) -> bool) -> usize {
    count_selected_cities_general(cities, test_fn)
}

/// An example of a test function. Note that the type of this function is `fn(&City) -> bool`, the
/// same as the `test_fn` argument to `count_selected_cities`.
pub(crate) fn has_monster_attacks(city: &City) -> bool {
    city.monster_attack_risk > 0.0
}

/// Surprisingly, closures donot have the same type as functions. To support closures, we must change
/// the type signature of this function. It needs to look as below:
/// It takes a test_fn of any type F as long as F implements the special trait Fn(&City) -> bool. This
/// trait is automatically implemented by all functions and most closures that take a single &City as
/// an argument and return a Boolean value:
///     fn(&City) -> bool   // fn type (functions only)
///     Fn(&City) -> bool   // Fn type (both functions and closures)
/// This special syntax is built into the language. The -> and return type are optional; if omitted,
/// the return type is ().
/// A closure is callable, but it's not a fn. In fact, every closure you write has its own type,
/// because a closure may contain data: values either borrowed or stolen from enclosing scopes.
/// This could be any number of variables, in any combination of types. So every closure has an ad
/// hoc type created by the compiler, large enough to hold that data. No two closures have exactly
/// the same type. But every closure implements an Fn trait;
/// Since every closure has its own type, code that works with closures usually needs to be generic.
pub(crate) fn count_selected_cities_general<F>(cities: &Vec<City>, test_fn: F) -> usize
    where F: Fn(&City) -> bool
{
    let mut count = 0;
    for city in cities {
        if test_fn(city) {
            count += 1;
        }
    }
    count
}


/// This generic function may be passed any closure that implements the trait Fn(): that is, closures
/// that take no arguments and return ().
pub(crate) fn call_twice<F>(mut closure: F) where F: FnMut() {
    closure();
    closure();
}

/// Closures that drop values are not allowed to have Fn. They are, quite literally, no Fn at all.
/// They implement a less powerful trait, FnOnce, the trait of closures that can be called once.
/// The first time you call a FnOnce closure, the closure itself is used up. It's as though the two
/// traits, Fn and FnOnce, were defined like this:
///
///     trait Fn() -> R {
///         fn call(&self) -> R;
///     }
///
///     trait FnOnce() -> R {
///         fn call_once(self) -> R;
///     }
/// There is one more kind of closure, the kind that contains mutable data or mut references. Therefore,
/// Rust has one more category of closure, FnMut, the category of closures that write. FnMut closures
/// are called by mut reference, as if they were defined like this:
///
///     trait FnMut() -> R {
///         fn call_mut(&mut self) -> R;
///     }
/// Any closure that requires mut access to a value, but doesn't drop any values, is an FnMut closure.
///
/// * Fn is the family of closures and functions that you can call multiple times without restriction.
/// This highest category also includes all fn functions.
/// * FnMut is the family of closures that can be called multiple times if the closure itself is
/// declared mut.
/// * FnOnce is the family of closures that can be called once, if the caller owns the closure.
///
/// Every Fn meets the requirements for FnMut, and every FnMut meets the requirements for FnOnce.
/// Fn() is a subtrait of FnMut(), which is a subtrait of FnOnce(). This makes Fn the most exclusive
/// and most powerful category. FnMut and FnOnce are broader categories that include closures with
/// usage restrictions.
///
/// Copy and Clone for Closures
/// A non-move closure that doesn't mutate variables holds only shared references, which are both
/// Copy and Clone, so that closure is both Clone and Copy as well.
/// On the other hand, a non-move closure that does mutate values has mutable references within its
/// internal representation. Mutable references are neither Clone nor Copy, so neither is a closure
/// that uses them.
struct Dummy;


