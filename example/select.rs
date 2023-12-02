#[allow(unused)]
use dialogue_macro::Dialogue;

#[derive(Dialogue)]
struct User {
    #[dialogue(prompt="请选择你的学校",options=["清华大学","北京大学","上海交通大学"],default="清华大学")]
    school: String,
}

fn main() {
    let school = User::school();
    println!("school:{}", school);
}
