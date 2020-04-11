const DIFF: i32 = 32;
const CELSIUS_FACTOR: f64 = 1.8;

fn main() {
    let unit = loop {
        let mut units = String::new();

        println!("(c)elsius or (f)arrenheit?");

        std::io::stdin()
            .read_line(&mut units)
            .expect("Failed to read line");
        units = String::from(units.trim()).to_lowercase();
        if units == "c" || units == "f" {
            break units;
        } else {
            continue;
        }
    };

    let temp: i32 = loop {
        let mut response = String::new();

        println!("What's the temperature you wish to convert?");

        std::io::stdin()
            .read_line(&mut response)
            .expect("Failed to read line");
        match response.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };
    if unit == "c" {
        println!("Temp is {} farrenheit", convert_to_farrenheit(temp));
    } else if unit == "f" {
        println!("Temp is {} celsius", convert_to_celsius(temp));
    }
}

fn convert_to_celsius(temp: i32) -> i32 {
    let diff = temp - DIFF;
    (diff as f64 / CELSIUS_FACTOR).round() as i32
}

fn convert_to_farrenheit(temp: i32) -> i32 {
    (temp as f64 * CELSIUS_FACTOR + DIFF as f64).round() as i32
}

#[test]
fn converts_to_celsius() {
    assert_eq!(-46, convert_to_celsius(-50));
    assert_eq!(-18, convert_to_celsius(0));
    assert_eq!(10, convert_to_celsius(50));
}

#[test]
fn converts_to_farrenheit() {
    assert_eq!(-58, convert_to_farrenheit(-50));
    assert_eq!(32, convert_to_farrenheit(0));
    assert_eq!(122, convert_to_farrenheit(50));
}
