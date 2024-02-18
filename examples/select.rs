#![allow(unused)]
use dialogue_macro::Asker;

#[derive(Asker, Debug)]
struct User {
    #[select(prompt = "Please select you sex", options = ["Male", "Female", "Other"], default = 1)]
    sex: String,
    #[select(prompt = "Please select you school: ", default = 1,options=[
        School{
            name: "清华大学".to_string(),
        },
        School{
            name: "北京大学".to_string(),
        },
        School{
            name: "河南理工大学".to_string(),
        }
    ],with_default=true)]
    school: Option<School>,
}

#[derive(Debug, Clone)]
struct School {
    name: String,
}

impl ToString for School {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

fn main() {
    let user = User::asker().sex().school(2).finish();

    println!("{:?}", user);
}
