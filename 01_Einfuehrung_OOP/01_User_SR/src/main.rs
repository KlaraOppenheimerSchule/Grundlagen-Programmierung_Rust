mod user;

fn main() {
    let mut user_one = user::User::new(String::from("TestUser"), String::from("TestPass"));

    println!("Username: {}", user_one.get_username());
    println!("Password: {}", user_one.get_password());

    println!("Change occurs!");
    user_one.set_username(String::from("SomeOtherUsername"));
    user_one.set_password(String::from("AnotherPassword"));

    println!("Username: {}", user_one.get_username());
    println!("Password: {}", user_one.get_password());

}
