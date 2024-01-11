use dialogue_macro::Asker;
#[derive(Debug, Asker)]
#[theme(colorful_macro)]
struct User {
    name: String,
    age: u32,
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

// impl User {
//     fn asker() -> UserAsker {
//         UserAsker::default()
//     }
// }

// struct UserAsker {
//     name: Option<String>,
//     age: Option<u32>,
//     email: Option<String>,
//     favorite: Vec<String>,
// }

// impl Default for UserAsker {
//     fn default() -> Self {
//         Self {
//             name: None,
//             age: None,
//             email: None,
//             favorite: vec!["5".to_string()],
//         }
//     }
// }

fn main() {
    let mut asker=User::asker();
    asker.boy();
    asker.age("你多少岁了").name("你叫什么名字").favorite("你最喜欢的数字是多少");
    let user=asker.finish();
    println!("{:#?}",user);
}
