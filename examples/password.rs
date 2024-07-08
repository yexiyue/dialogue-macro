#![allow(unused)]
use dialogue_macro::Asker;

#[derive(Asker, Debug)]
struct User {
    // 普通密码输入字段，用户可选择是否输入密码
    #[password(
        validate_with = |input| {
            if input.len() < 6 {
                return Err("密码长度不能小于6");
            }
            Ok(())
        }
    )]
    password: Option<String>,

    // 带确认和错误提示的密码输入字段
    #[password(
        prompt = "输入密码(测试2):",
        confirmation = "再次输入密码(测试2):",
        mismatch = "两次输入的密码不匹配"
    )]
    password2: String,
}

fn main() {
    let user = User::asker().password("输入密码:").password2().finish();

    println!("{:?}", user);
}
