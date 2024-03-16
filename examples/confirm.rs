#![allow(unused)]
use dialogue_macro::Asker;

#[derive(Asker, Debug)]
struct User {
    boy: bool,

    #[confirm(prompt = "Are you a student?", with_default = true)]
    student: Option<bool>,
}

fn main() {

    let user = User::asker()
        .boy("Are you a boy?")
        .student(true)
        .finish();

    println!("{:?}", user);
}
