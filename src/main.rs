fn main() {
    let number = 3;
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    println!("The value of this is {}", conditional());
    a_loop();
    lift_off();
    for_loop();
    a_range();
}

fn conditional() -> i32 {
    let condition = false;
    let number = if condition { 5 } else { 6 };
    number
}

fn a_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            // Result gets assigned the value of the break return
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn lift_off() {
    let mut counter = 3;

    while counter != 0 {
        println!("{}!", counter);
        counter -= 1;
    }

    println!("LIFT OFF!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }
}

fn a_range() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("Lifted off with cleaner code!");
}
