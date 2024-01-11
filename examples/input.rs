#![allow(unused)]
use dialogue_macro::Asker;

#[derive(Asker, Debug)]
struct User {
    name: String,
    #[input(prompt = "Enter your email address: ")]
    email: String,
    #[input(default = 16)]
    age: u8,
    #[input(prompt = "Enter your school: ", default = "清华大学")]
    school: Option<String>,
}

fn main() {
    let user = User::asker()
        .name("Enter your name: ")
        .email()
        .age("Enter your age: ")
        .school()
        .finish();

    println!("{:?}", user);
}
