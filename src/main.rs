#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.area() > rect.area()
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rectangle = Rectangle {
        height: 30,
        width: 50,
    };

    let small_rect = Rectangle {
        height: 20,
        width: 30,
    };

    let square = Rectangle::square(20);

    println!("The area of the rectangle is {}", rectangle.area());
    println!("Rectangle is {:?}", rectangle);
    println!("And even prettier it is {:#?}", rectangle);
    println!(
        "Rectangle 1 can hold Rectangle 2 is {}",
        rectangle.can_hold(&small_rect)
    );
    println!("Our square {:?}", square);
}
