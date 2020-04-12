struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user = User {
        email: String::from("jonathan@ingallnet.co.uk"),
        username: String::from("ingalless"),
        active: true,
        sign_in_count: 1,
    };

    // because the struct is mutable, the fields are mutable too
    user.email = String::from("jennifer@ingallnet.co.uk");

    let user2 = build_user(String::from("jonny@ingall.com"), String::from("ingalless"));
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
