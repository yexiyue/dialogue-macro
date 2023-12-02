use dialogue_core::Dialogue;

#[derive(Dialogue)]
struct User {
    name: String,
    #[dialogue(options=["a","b","c"])]
    age: Vec<String>,
    ok: bool,
}

fn main() {}
