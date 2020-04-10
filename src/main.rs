fn main() {
    let number = 3;
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    println!("The value of this is {}", conditional());
}

fn conditional() -> i32 {
    let condition = false;
    let number = if condition { 5 } else { 6 };
    number
}
