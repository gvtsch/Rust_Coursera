#[derive(Debug)]

struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }
    fn deactivate(&mut self) {
        self.active = false;
    }

    fn from_email(email: &str) -> User {
        let username = email.split('@').next().unwrap().to_string();
        User {
            username,
            email: email.to_string(),
            uri: String::new(),
            active: true,
        }
    }

    fn update_uri(&mut self, new_uri: &str) {
        self.uri = new_uri.to_string();
    }
         
}

fn main() {
    let mut new_user = User::new(
        String::from("alfredodeza"),
        String::from("alfreodeza@example.com"),
        String::from("https://alfredodeza.com"),
    );
    println!("Hello, {}!", new_user.username);
    println!("Account {} status is: {}", new_user.username, new_user.active);
    new_user.deactivate();
    println!("Account {} status is: {}", new_user.username, new_user.active);

    let mut user = User::from_email("john@example.com");
    println!("{:?}", user); // Prints: User { username: "john", email: "john@example.com", uri: "", active: true }

    user.update_uri("https://new.uri");
    println!("{:?}", user); // Prints: User { username: "john", email: "john@example.com", uri: "https://new.uri", active: true }

}
