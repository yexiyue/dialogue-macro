#![allow(unused)]
use dialogue_macro::Dialogue;

#[derive(Dialogue, Debug)]
struct Password {
    #[dialogue(password = true)]
    password: String,
    #[dialogue(prompt = "请输入密码", password = true)]
    password2: String,
    #[dialogue(
        prompt = "请输入密码",
        password = true,
        confirmation = "请再次输入密码"
    )]
    password3: String,
    #[dialogue(
        prompt = "请输入密码",
        password = true,
        confirmation = "请再次输入密码",
        mismatch = "两次输入的密码不一致"
    )]
    password4: String,
}

fn main() {
    let p1 = Password::password("请输入密码");
    let p2 = Password::password2();
    let p3 = Password::password3();
    let p4 = Password::password4();
    let password = Password {
        password: p1,
        password2: p2,
        password3: p3,
        password4: p4,
    };
    println!("{:?}", password);
}
