fn main() {
    let stack_s = "Hello"; // This gets put on the stack
    let mut s = String::from("Hello"); // This gets put on the heap
    s.push_str(", world!");
    println!("{}", s);
}

fn pointers() {
    // If we really want to copy the data too, then you can use clone()!
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("one {} and two {}", s1, s2);
}
