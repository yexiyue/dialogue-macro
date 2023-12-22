#![allow(unused)]

use dialogue_macro::Dialogue;
#[derive(Dialogue)]
struct User {
    #[dialogue(prompt="请选择你意向的学校",options=["清华大学","北京大学","上海交通大学"],default=["清华大学","北京大学","上海交通大学"])]
    school: Vec<String>,
}

fn main() {
    let school = User::school();
    println!("school:{:?}", school);
}
