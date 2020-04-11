fn main() {
    let stack_s = "Hello"; // This gets put on the stack
    let mut s = String::from("Hello"); // This gets put on the heap
    s.push_str(", world!");
    println!("{}", s);
}

fn pointers() {
    // When we do the below, because the values get pushed to the stack, we can just push 2 individual values.
    // Namely, x = 5, y = 5
    let x = 5;
    let y = x;

    // But in this case, s1 would go on the heap. It would be expensive to push both on the heap.
    // So instead, s2 will copy the pointer, length and capacity from s1, but NOT the data.
    let s1 = String::from("Hello");
    let s2 = s1;

    // But something else important happens. The reference to s1 gets lost.
    // Therefore, we can't do "println!("{}", sl);" because the ownership has moved to s2 now.
}
