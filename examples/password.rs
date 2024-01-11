#![allow(unused)]
use dialogue_macro::Asker;

#[derive(Asker, Debug)]
struct User {
    #[password()]
    password: Option<String>,
    #[password(
        prompt = "Enter your password:",
        confirmation = "Enter your password again:",
        mismatch = "Your passwords do not match"
    )]
    password2: String,
}

fn main() {
    let user = User::asker().password2().password("Enter your password:").finish();
    println!("{:?}", user);
}
