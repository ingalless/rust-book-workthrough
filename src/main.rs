fn main() {
    println!("Integers");
    integers();

    println!("Floating points");
    floating_point();

    println!("Expressions");
    expression();

    println!("Awesome character types");
    awesome_char_type();

    println!("Arrays and Tuples");
    array_vs_tuple();
}

fn integers() {
    let eight_bit: u8 = 255;
    println!("Eight bit max: {}", eight_bit);

    let sixteen_bit: u16 = 65535;
    println!("Sixteen bit max: {}", sixteen_bit);

    let default_bit = 1234568;
    println!("The default is u32: {}", default_bit);
}

fn floating_point() {
    let x = 2.0; // default f64
    let y: f32 = 3.0;
    println!("X equals {} and y = {}", x, y);
}

fn expression() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let tax = 140 / 20;

    // remainder
    let remainder = 43 % 5;

    println!("5 + 10 = {}", sum);
    println!("95.5 - 4.3 = {}", difference);
    println!("4 x 30 = {}", product);
    println!("Tax of item priced 140.20 = {}", tax);
    println!("Remainder of 43 / 5 = {}", remainder);
}

fn awesome_char_type() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{} {} {}", c, z, heart_eyed_cat);
}

fn array_vs_tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // destruct
    let (x, y, z) = tup;
    println!("The value of y is {}", y);

    // or directly
    println!("The value of x is {}", tup.0);

    // Arrays are fixed length!!
    let array = ["Jonny", "Jimmy", "Jenny"];
    println!("Jonny is {}", array[0]);

    // Trying to access an index outside of the array terminates the program to prevent memory leaks!
    // array[5] wouldn't be allowed, for example.
}
