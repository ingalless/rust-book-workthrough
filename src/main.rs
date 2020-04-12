struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Although the above tuples are identical in signature,
    // you wouldn't be able to use a Point instead of a Color in a function
}
