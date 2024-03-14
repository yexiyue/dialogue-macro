#![allow(unused)]

use dialogue_core::EnumAsker;
use dialogue_macro::{Asker, Build};

#[derive(Debug, Asker)]
#[asker(theme = "dialogue_macro::ColorfulTheme")]
struct UserInput {
    username: String,
    is_student_status: bool,
    educational_institution: String,

    #[asker(SubAsker)]
    detailed_info: DetailedUserInfo,
    #[asker(SubAsker)]
    choice: EnumSelection,
}

#[derive(Debug, Clone, Asker)]
struct DetailedUserInfo {
    email_address: String,
    age: u8,
}

#[derive(Debug, EnumAsker, Clone)]
enum EnumSelection {
    OptionD,
    OptionE,
    OptionF,
}

impl Build for DetailedUserInfo {
    fn build() -> Self {
        Self::asker()
            .email_address("请提供您的邮箱地址")
            .age("请输入您的年龄")
            .finish()
    }
}

fn main() {
    let collected_user_input = UserInput::asker()
        .username("请输入用户名")
        .is_student_status("您是否是学生身份?")
        .educational_institution("请输入您的学校名称")
        .detailed_info()
        .choice()
        .finish();

    println!("{:?}", collected_user_input);
}
