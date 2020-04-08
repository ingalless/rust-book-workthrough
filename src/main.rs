fn main() {
    println!("Mutable...");
    mutable();
    println!("Shadowing");
    shadowed();
    println!("Of different types");
    of_dif_types();
}

fn mutable() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn shadowed() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}

fn of_dif_types() {
    let spaces = "    ";
    let spaces = spaces.len();
    println!("Spaces length is: {}", spaces);
}
