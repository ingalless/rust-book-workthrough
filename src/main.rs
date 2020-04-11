fn main() {
    let s = String::from("Hello world");

    println!("{}", first_word(&s));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..index];
        }
    }

    &s[..]
}
