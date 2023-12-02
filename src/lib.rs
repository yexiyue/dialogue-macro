use dialogue_core::dialogue_define;
mod theme;
pub use dialogue_core::Dialogue;
pub use dialoguer;
pub use theme::ColorfulTheme;

fn main() {
    dialogue_define!({
        A<T>=>{
            prompt: "请输入A的值",
            default: "1",
            confirmation: "请输入A的值",
            options: ["1", "2", "3"],
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
}

