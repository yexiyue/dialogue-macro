use dialogue_macro::Dialogue;

#[derive(Dialogue)]
struct User {
    #[dialogue(default = "wasm-project", prompt = "请输入项目名称" ,options=["wasm-project","wasm-project2"])]
    name: String,
}

fn main() {}
