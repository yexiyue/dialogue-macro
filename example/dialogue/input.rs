#![allow(unused)]
use dialogue_macro::Dialogue;
#[derive(Dialogue, Debug)]
struct User {
    name: String,
    age: u32,
    #[dialogue(prompt = "请输入你的邮箱")]
    email: String,
    #[dialogue(prompt = "请输入你的年级", default = "5")]
    grade: i32,
}

fn main() {
    let name = User::name("请输入你的名字");
    let age = User::age("请输入你的年龄");
    let email = User::email();
    let grade = User::grade();
    let user = User {
        name,
        age,
        email,
        grade,
    };

    println!("{:#?}", user);
}
