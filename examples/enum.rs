#![allow(unused)]

use dialogue_macro::{Asker, Build, EnumAsker};

#[derive(Debug, Asker, Clone)]
struct User {
    #[input(with_default = true)]
    username: String,
    email: String,
}

impl Build for User {
    fn build() -> Self {
        Self::asker()
            .username("Enter your username", "default_username".to_string())
            .email("Enter your email")
            .finish()
    }
}

#[derive(Debug, Asker, Clone)]
pub struct Student {
    username: String,
    student_id: String,
}

impl Build for Student {
    fn build() -> Self {
        Self::asker()
            .username("Enter your username")
            .student_id("Enter your student ID")
            .finish()
    }
}

#[derive(Debug, EnumAsker)]
#[asker(
    prompt = "Select registration type",
    default = "Student",
    theme = "dialoguer::theme::ColorfulTheme"
)]
enum Register {
    #[asker(label = "Register as a regular user")]
    User(User),
    #[asker(label = "Register as a student")]
    Student(Student),
    #[asker(label = "Exit")]
    Exit,
}

fn main() {
    let choice = Register::build();
    match choice {
        Register::User(user) => println!("Registered regular user: {:?}", user),
        Register::Student(student) => println!("Registered student: {:?}", student),
        Register::Exit => println!("Exiting..."),
    }
}
