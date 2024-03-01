#![allow(unused)]
use dialogue_macro::Asker;

#[derive(Asker, Debug)]
struct User {
    #[asker(skip)]
    name: Option<String>,
    #[input(prompt = "Enter your email address: ", with_default = true)]
    email: String,
    #[input(default = 16,with_default = true)]
    age: u8,
    #[input(prompt = "Enter your school: ", default = "清华大学")]
    school: Option<String>,
}

fn main() {
    let user = User::asker()
        // .name("Enter your name: ")
        .email("yexiyue666@qq.com".into())
        .age("Enter your age: ",17)
        .school()
        .finish();

    println!("{:?}", user);
}
