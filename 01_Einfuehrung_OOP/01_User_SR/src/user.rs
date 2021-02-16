#[derive(Default)] // Automatically implement Default values
pub struct User {
    username: String,
    password: String,
}

impl User {
    pub fn new(username: String, password: String) -> User {
        User {
            username: username,
            password: password,
        }
    }
    pub fn get_username(&self) -> String {
        self.username.clone()
        // oder
        // return self.username;
    }

    pub fn get_password(&self) -> String {
        self.password.clone()
        // oder
        // return self.password;
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }
}
