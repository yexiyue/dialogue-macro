#![allow(unused)]
use dialogue_core::Asker;

#[derive(Asker, Debug)]
// #[asker(theme = "dialogue_macro::dialoguer::theme::ColorfulTheme")]
struct User {
    #[input(prompt = "Enter your name:")]
    name: String,
    age: u32,
    #[password(prompt = "Enter your password:")]
    password: String,
    email: Option<String>,
    #[select(prompt="Please select you sex", options = ["Male", "Female", "Other"], default = 1)]
    sex: String,
    #[confirm(prompt = "Are you sure?")]
    sure: bool,
    #[multiselect(prompt = "Please select you favorite",default=[1])]
    favorite: Vec<String>,
}

fn main() {
    let favorite = vec!["eat".to_string(), "sleep".to_string(), "code".to_string()];
    let user = User::asker()
        .name()
        .age("Enter your age:")
        .email("Enter your email:")
        .password()
        .sex()
        .favorite(&favorite)
        .sure()
        .finish();
    println!("{:?}", user);
}
