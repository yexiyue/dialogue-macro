#[allow(unused)]
use dialogue_macro::Dialogue;

#[derive(Dialogue)]
struct User {
    name: String,
    #[dialogue(password = true, prompt = "请输入密码")]
    password: String,
}

fn main() {
    let username = User::name("你的名字?");
    let password = User::password();

    println!("你好 {}, 你的密码是 {}", username, password);
}
