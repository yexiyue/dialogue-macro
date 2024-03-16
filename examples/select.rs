#![allow(unused)]
use dialogue_macro::Asker;

#[derive(Asker, Debug)]
struct User {
    
    #[select(prompt = "Please select your sex", options = ["Male", "Female", "Other"], default = 1)]
    sex: String,

    #[select(
        prompt = "Please select your school: ",
        default = 1,
        options = [
            School { name: "Tsinghua University".to_string() },
            School { name: "Peking University".to_string() },
            School { name: "Henan Polytechnic University".to_string() }
        ],
        with_default = true
    )]
    school: Option<School>,


    #[select()]
    favorite: String,
}

// 定义表示学校的结构体，实现Debug和Clone特质
#[derive(Debug, Clone)]
struct School {
    name: String,
}

// 实现ToString trait，将School结构体转换为字符串形式
impl ToString for School {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

fn main() {

    let options = vec!["Eat".to_string(), "Sleep".to_string(), "Coding".to_string()];

    let user = User::asker()
        .sex()
        .school(2)
        .favorite("Please select your favorite:", &options)
        .finish();

    println!("{:?}", user);
}
