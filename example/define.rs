// fn main(){
//     let mut p1 = Person{
//         name:String::from("John"),
//         age:20,
//         gender:String::from("Male"),
//         hobbies:vec![String::from("Swimming"),String::from("Reading")]
//     };

//     p1.to_string();
// }
use dialogue_core::dialogue_define;

#[derive(Debug,Clone)]
struct Person {
    name: String,
    age: i32,
    gender: String,
    hobbies: Vec<String>,
}

impl ToString for Person {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

dialogue_define!({
    A<Person>=>{
        prompt: "请输入A的值",
        default: "1",
        confirmation: "请输入A的值",
        options: ["1", "2", "3"],
        ty:"multiselect"
    },
    B=>{
        prompt: "请输入B的值",
        default: "2",
        confirmation: "请输入B的值",
        options: ["1", "2", "3"],
    },
    C=>{
        prompt: "请输入C的值",
        default: "3",
    }
});

fn main() {
    let d=Dialogue::new();
    println!("{:#?}",d);
}
