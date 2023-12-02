use dialogue_core::Dialogue;

#[derive(Dialogue)]
struct User {
    #[dialogue(default1 = "wasm-project", prompt = "请输入项目名称" ,options=["wasm-project","wasm-project2",123], multi_select = true)]
    name: String,
    age: i32,
}

fn main() {}
