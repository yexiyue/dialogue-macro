#![allow(unused)]

use dialogue_macro::Asker;
#[derive(Debug, Asker)]
#[asker(theme = "dialogue_macro::ColorfulTheme")]
struct User {
    name: String,
    age: u32,
    #[password(prompt="请输入密码")]
    email: Option<String>,
    #[multiselect(options=[ 
        Person{name:"5".to_string(),age:10},
        Person{name:"6".to_string(),age:11},
        Person{name:"7".to_string(),age:12}
    ])]
    favorite: Vec<Person>,
    #[confirm(default = true,prompt="你是男孩吗")]
    boy: bool,
}

#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
}

impl ToString for Person {
    fn to_string(&self) -> String {
        format!("{} {}", self.name, self.age)
    }
}


fn main() {
    let mut asker=User::asker();
    // asker.boy();
    // asker.age("你多少岁了").name("你叫什么名字").favorite("你最喜欢的数字是多少").email();
    // let user=asker.finish();
    // println!("{:#?}",user);
}
