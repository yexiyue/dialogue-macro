#[allow(unused)]
use dialogue_macro::Dialogue;
use dialogue_macro::ColorfulTheme;
#[derive(Dialogue)]
struct User {
    name: String,
    #[dialogue(prompt = "请输入父母的名字")]
    parent: String,
    #[dialogue(password = true, prompt = "请输入密码")]
    password: String,
}

fn main() {
    let username = User::name("你的名字?");
    let parent = User::parent();
    let password = User::password();
    
    println!(
        "你好 {},你的父母是 {} 你的密码是 {}",
        username, parent, password
    );
}
