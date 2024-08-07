#![allow(unused)]
use dialogue_macro::Asker;

#[derive(Asker, Debug)]
struct User {
    name: Option<String>,
    #[input(prompt = "请输入您的邮箱地址: ", with_default = true)]
    email: String,
    #[input(with_default = true)]
    age: u8,
    #[input(
        prompt = "请输入您的学校名称: ",
        with_default = true,
        validate_with = |input| {
            if input.contains("大学") { Ok(()) } else { Err("学校名称必须包含大学") }
        }
    )]
    school: Option<String>,
}

fn main() {
    let user = User::asker()
        .name("请输入您的姓名: ")
        .email("yexiyue666@qq.com")
        .age("请输入您的年龄", 17)
        .school("北京大学")
        .finish();

    println!("{:?}", user);
}
