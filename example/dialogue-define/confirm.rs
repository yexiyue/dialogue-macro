#![allow(unused)]
use dialogue_macro::dialogue_define;
// 使用变量的形式定义参数
static PROMPT: &str = "你是小学生吗？";
static DEFAULT: bool = true;

// 自动生成一个Dialogue结构体，并实现相关的方法
dialogue_define!({
    student=>{
        ty:"confirm",
        prompt: PROMPT,
        default: DEFAULT,
    },
    teacher=>{
        prompt: "你是老师吗？",
        default: "true",
        ty:"confirm"
    }
});
// 使用别名
type User = Dialogue;

fn main() {
    let mut p1 = User::new();
    p1.student();
    p1.teacher();
    println!("{:?}", p1);
}
