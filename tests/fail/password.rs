#![allow(unused)]
use dialogue_macro::Asker;

#[derive(Asker, Debug)]
struct User {
    #[password(
        prompt = "Enter your password:",
        confirm = "Enter your password again:",
        mismatch = "Your passwords do not match"
    )]
    password2: String,
    #[password()]
    password: Option<bool>,
}

fn main() {
    let user = User::asker()
        .password2()
        .password("Enter your password:")
        .finish();
    println!("{:?}", user);
}
