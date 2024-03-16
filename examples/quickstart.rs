#![allow(unused)]
use dialogue_macro::Asker;
#[derive(Asker, Debug)]
struct User {
    #[input(prompt = "Enter your name:")]
    name: String,
    age: u32,
}

fn main() {
    let user = User::asker().name().age("Enter your age:").finish();
    println!("{:?}", user);
}
