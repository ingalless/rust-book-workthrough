fn main() {
    another_function(10);
    add_numbers(10, 5);
    create_new_scope();
    let result = add_numbers_and_get_result(10, 2);
    println!("Result of returned function is {}", result);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn add_numbers(a: i32, b: i32) {
    println!("The result of {} + {} is {}", a, b, a + b);
}

fn add_numbers_and_get_result(a: i32, b: i32) -> i32 {
    a + b
}

fn create_new_scope() {
    // {} create new scopes
    let x = 5;

    let y = {
        let x = 3;
        // To return, don't end on a semicolon!
        x + 1
    };

    // Notice x is 5? The scope of x in our y is different!
    println!("Y is {} and x is {}", y, x);
}
