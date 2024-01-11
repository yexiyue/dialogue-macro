#![allow(unused)]
use dialogue_macro::Asker;

#[derive(Asker, Debug)]
struct User {
    boy: bool,
    #[confirm(prompt = "Are you a student?")]
    student: Option<bool>,
}

fn main() {
    let user = User::asker().boy("你是男孩吗？").student().finish();
    println!("{:?}", user);
}
