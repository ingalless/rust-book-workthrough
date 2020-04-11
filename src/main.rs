fn main() {
    let s = String::from("Hello");

    let len = calculate_length(&s);

    let mut mutatable_s = String::from("Hello");
    let mutated_len = modify_and_calculate_length(&mut mutatable_s);

    println!(
        "The length of '{}' is {} and mutatable is '{}' with length of {}",
        s, len, mutatable_s, mutated_len
    );
}

// In the below function s is "borrowed" from the calling scope...
fn calculate_length(s: &String) -> usize {
    s.len()
}
// So drop isn't called here, meaning it's still in scope in main()

// This example is the same as above, only we're mutating the variable passed in
fn modify_and_calculate_length(s: &mut String) -> usize {
    s.push_str(", world");
    s.len()
}
// This will mutate the value in the calling scope
