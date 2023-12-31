struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        User {
            username,
            email,
            uri,
            active: true,
        }
    }
    fn deactivate(&mut self) {
        self.active = false;
    }
}

fn main() {
    let mut new_user = User::new(
        String::from("John Doe"),
        String::from("johndoe@mail.com"),
        String::from("https://johndoe.com")
    );
    println!("Hello, {}!", new_user.username);
    println!("Account {} status: {}", new_user.username, new_user.active);
    new_user.deactivate();
    println!("Account {} status: {}", new_user.username, new_user.active);
}
