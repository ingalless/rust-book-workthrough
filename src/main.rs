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
    let user3 =
        build_new_from_existing_user(String::from("email"), String::from("username"), user2);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// Below illustrates update syntax, kind of like javascript object spread!
fn build_new_from_existing_user(email: String, username: String, user: User) -> User {
    User {
        email,
        username,
        ..user
    }
}
