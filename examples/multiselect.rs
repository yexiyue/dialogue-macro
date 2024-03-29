#![allow(unused)]
use dialogue_macro::Asker;

#[derive(Asker, Debug)]
#[asker(theme = "dialoguer::theme::ColorfulTheme")]
struct User {
    #[multiselect(prompt = "Please select your favorite", options = ["Eat", "Sleep", "Code"], default = [1])]
    favorite: Vec<String>,

    #[multiselect(
        prompt = "Please select your school: ",
        with_default = true,
        options = [
            School { name: "Tsinghua University".to_string() },
            School { name: "Peking University".to_string() },
            School { name: "Henan Polytechnic University".to_string() }
        ]
    )]
    school: Vec<School>,
}

// 定义表示学校的结构体，并实现Debug和Clone特质
#[derive(Debug, Clone)]
struct School {
    name: String,
}

// 实现ToString trait，将School结构体转换为字符串形式以便于展示
impl ToString for School {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

fn main() {
    let user = User::asker()
        .school(&[true, false, true])
        .favorite()
        .finish();

    println!("{:?}", user);
}
