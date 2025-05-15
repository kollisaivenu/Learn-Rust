struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

struct Color(i32, i32, i32);
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width*self.height
    }
}

impl Rectangle{
    fn square(size: u32) -> Rectangle {
        Rectangle{
            width: size,
            height: size
        }
    }
}

fn main() {
    let user1 = User{
        username: String::from("abc"),
        email: String::from("abc@gmail.com"),
        active: true,
        sign_in_count: 1
    };

    let name: String = user1.username;

    let mut user2 = User{
        username: String::from("abc"),
        email: String::from("abc@gmail.com"),
        active: true,
        sign_in_count: 1
    };
    user2.email = String::from("efg@gmail.com");

    let mut user3 = build_user(String::from("xyz"), String::from("xyz@gmail.com"));

    let user4 = User{
        username: String::from("rst"),
        ..user3
    };

    // Values of user3 such as email address are moved.
    let rect = Rectangle{
        width:5,
        height:6
    };
    println!("rect:{rect:?}");
    println!("rect area: {}", rect.area());
    let sqr = Rectangle::square(3);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1
    }
}


