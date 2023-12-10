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

#[derive(Debug, Clone)]
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

static PERSON_PROMPT: &str = "请输入个人信息";
static OPTIONS: [&str; 3] = ["1", "2", "3"];
static DEFAULT: &str = "3";
static DEFAULT_TRUE: bool = true;
dialogue_define!({
    // A<Person>=>{
    //     prompt: "请输入A的值",
    //     default: "1",
    //     confirmation: "请输入A的值",
    //     options: ["1", "2", "3"],
    //     ty:"select"
    // },
    B=>{
        prompt: "请输入B的值",
        default: DEFAULT,
        confirmation: "请输入B的值",
        options: OPTIONS,
        ty:"select"
    },
    C=>{
        // prompt: "请输入C的值",
        default: PERSON_PROMPT,
    },
    D=>{
        ty:"confirm",
        prompt: "D是对滴",
        default: DEFAULT_TRUE,
    },
    E=>{
        ty:"password",
        confirmation: "请再次输入密码",
        mismatch: "两次输入的密码不一致",
        default:"123456"
    },
    F<i32>=>{

    },
    H=>{
        prompt:PERSON_PROMPT,
        options:OPTIONS,
        ty:"select"
    }
});

fn main() {
    let mut d = Dialogue::new();
    d.B();
    d.C("hello world");
    d.D();
    // d.E("请输入密码");
    // d.F("请输入数字");
    // d.H();
    // let persons = vec![
    //     Person {
    //         name: String::from("John"),
    //         age: 20,
    //         gender: String::from("Male"),
    //         hobbies: vec![String::from("Swimming"), String::from("Reading")],
    //     },
    //     Person {
    //         name: String::from("Jane"),
    //         age: 21,
    //         gender: String::from("Female"),
    //         hobbies: vec![String::from("Swimming"), String::from("Reading")],
    //     },
    // ];
    // let res = dialoguer::Select::with_theme(&dialogue_macro::ColorfulTheme::default())
    //     .with_prompt("请输入B的值")
    //     .items(&persons)
    //     .default("1".parse().unwrap())
    //     .interact()
    //     .unwrap();
    // println!("{}", res);
    
    println!("{:#?}", d);
}
