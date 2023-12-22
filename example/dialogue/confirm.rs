#![allow(unused)]
use dialogue_macro::Dialogue;
#[derive(Debug, Dialogue)]
struct Person {
    student:bool,
    #[dialogue(prompt = "你是老师吗",default=false)]
    teacher:bool,
}
fn main() {
    let student=Person::student("你是学生吗？");
    let teacher=Person::teacher();
    let p1 = Person{
        student,
        teacher,
    };
    println!("{:?}", p1);
}
