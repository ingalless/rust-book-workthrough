fn main() {
    let s = String::from("hello"); // s comes into scope

    let s = takes_ownership_and_gives_back(s); // s's value moves into the function...
    println!("{}", s); // This can work now and is good because we minimise the work of the heap

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it’s okay to still
                   // use x afterward
    println!("{}", x);
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership_and_gives_back(some_string: &String) -> String {
    // some_string comes into scope
    println!("{}", some_string);
    some_string
} // We can give ownership back by returning!

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
