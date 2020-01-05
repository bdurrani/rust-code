struct User{
    username: String,
    email: String,
    sing_in_count:u64,
    active: bool,
}

fn main() {
    let mut user1 = User{
        email: String::from("some@example.com"),
        username: String::from("username"),
        active: true,
        sing_in_count: 1
    };
    user1.email = String::from("test@email.com");

    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 is {:#?}", rect1.area());
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sing_in_count: 1,
    }
}
