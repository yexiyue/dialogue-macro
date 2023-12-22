#![allow(unused)]
use dialogue_macro::dialogue_define;

static DEFAULT_AGE: i32 = 17;
static PROMPT: &str = "你的名字是什么";
dialogue_define!({
    name=>{
        ty:"input",
        prompt: PROMPT,
        default: "John"
    },
    age<i32>=>{
        default: DEFAULT_AGE
    }
});

fn main() {
    let mut user = Dialogue::new();
    user.name();
    user.age("你的年龄");

    println!("{:#?}", user);
}
