//------------------------------------------------------------------------------
// 1. Generic function

// Explicit, statically typed function
// fn swap(pair: (i32, &str)) -> (&str, i32) {
//     (pair.1, pair.0)
// }

// Generic function
fn swap<T, U>(pair: (T, U)) -> (U, T) {
    (pair.1, pair.0)
}

fn demo_swap() {
    let original = (42, "hello");
    let swapped = swap(original);

    println!("{:?}", swapped);
}

//------------------------------------------------------------------------------
// 2. Trait

// Define the trait
trait Hello {
    fn say_hello(&self);
}

// Define a type
struct Person {
    name: String,
}

// Implement the trait for the type
impl Hello for Person {
    fn say_hello(&self) {
        println!("Hello, {}", self.name);
    }
}

// Use the struct
fn demo_trait() {
    let me = Person {
        name: String::from("Shawn"),
    };

    me.say_hello();
}

//------------------------------------------------------------------------------
// 3. Trait bounds

// Error: cannot add `T` to `T`
// fn add<T>(a: T, b: T) -> T
// {
//     a + b
// }

// Fix: add trait bound
fn add<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b
}

fn demo_add() {

    // This works: T = i32
    let result_1 = add(-3, 10);
    println!("{}", result_1);

    // This works: T = f64
    let result_2 = add(12.345, 2.86);
    println!("{}", result_2);

    // Error: cannot add `bool` to `bool`
    // let result_3 = add(true, false);
    // println!("{:?}", result_3);

    // Error: cannot add `&str` to `&str` (Add<&str> not implemented for &str)
    // let result_4 = add("Hello, ", "world!");
    // println!("{:?}", result_4);
}

//------------------------------------------------------------------------------
// 4. Generic struct

struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {

    // Acts as a manual constructor
    fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }

    // Swap first and second
    fn swap(self) -> Pair<U, T> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }
}

fn demo_struct() {
    let collection = Pair::new(42, "hello");
    let swapped = collection.swap();

    println!("{}, {}", swapped.first, swapped.second);
}

//------------------------------------------------------------------------------
// 5. Generic enum

// Similar to Option<T> { Some(T), None }
enum Maybe<T> {
    Something(T),
    Nothing,
}

impl<T> Maybe<T> {

    // Panic if nothing, otherwise return the value in Something
    fn unwrap(self) -> T {
        match self {
            Maybe::Something(value) => value,
            Maybe::Nothing => panic!("Called unwrap on Nothing"),
        }
    }
}

fn demo_enum() {
    let no_value: Maybe<String> = Maybe::Nothing;
    let some_number: Maybe<f64> = Maybe::Something(1.2345);

    // Check manually
    match no_value {
        Maybe::Something(value) => println!("Value: {}", value),
        Maybe::Nothing => println!("No value found"),
    }

    // If we know that it's not nothing, we can use unwrap
    println!("Unwrapped value: {}", some_number.unwrap());
}

//------------------------------------------------------------------------------
// Main

fn main() {
    demo_swap();
    demo_trait();
    demo_add();
    demo_struct();
    demo_enum();
}