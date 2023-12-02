#[allow(unused)]
use dialogue_macro::Dialogue;
#[derive(Dialogue)]
struct User {
    #[dialogue(prompt = "是否是学生", default = false)]
    is_student: bool,
}

fn main() {
    let is_student = User::is_student();

    println!("is_student: {}", is_student);
}
