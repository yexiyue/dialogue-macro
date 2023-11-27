#[allow(unused)]
use dialogue_macro::Dialogue;

#[derive(Dialogue)]
struct User {
    age: isize,
    #[dialogue(default = 6, prompt = "你几年级了")]
    grade: i32,
}

fn main() {
    let age = User::age("你多少岁了");
    let grade = User::grade();
    println!("{}:grade {}", age, grade);
}
