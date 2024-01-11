#![allow(unused)]
use dialogue_macro::Asker;

#[derive(Asker, Debug)]
struct User {
    #[multiselect(prompt = "Please select you favorite", options = ["eat", "sleep", "code"],default=[1])]
    favorite: Vec<String>,
    #[multiselect(prompt = "Please select you school: ", default = [1,2],options=[
        School{
            name: "清华大学".to_string(),
        },
        School{
            name: "北京大学".to_string(),
        },
        School{
            name: "河南理工大学".to_string(),
        }
    ])]
    school: Vec<School>,
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
    let user = User::asker().school().favorite().finish();

    println!("{:?}", user);
}
