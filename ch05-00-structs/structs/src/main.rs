struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("hello@example.com"),
        username: String::from("hello"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;
    user1.username = String::from("hello2");

    let user2 = build_user(String::from("hello@example.com"), String::from("hello"));
    let user3 = User {
        email: String::from("hello@example.com"),
        username: String::from("hello"),
        ..user2
    };

    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    struct Point(i32, i32, i32);
    let origin = Point(0, 0, 0);

    let rect = (30, 50);

    println!("area is {}", area(rect));

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("area is {}", area2(&rect2));
    println!("rect {:#?}", rect2);

    println!("area is {}", rect2.area());

    let rect3 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("rect can hold rect3 {}", rect2.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("sq {:#?}", sq);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        // method
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        // method
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        // associated function
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // field init shorthand
        username,
        active: true,
        sign_in_count: 1,
    }
}
