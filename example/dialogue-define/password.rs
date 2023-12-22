#![allow(unused)]
use dialogue_macro::dialogue_define;
static PROMPT: &str = "请输入密码";
dialogue_define!({
    password1=>{
        ty:"password",
    },
    password2=>{
        ty:"password",
        prompt:PROMPT,
    },
    password3=>{
        ty:"password",
        prompt:PROMPT,
        confirmation:"请再次输入密码!",
    },
    password4=>{
        ty:"password",
        prompt:PROMPT,
        confirmation:"请再次输入密码#",
        mismatch:"两次输入的密码不一致#",
    }
});

fn main() {
    let mut password = Dialogue::new();
    password.password1("请输入密码呀");
    password.password2();
    password.password3();
    password.password4();
    println!("{:?}", password);
}
