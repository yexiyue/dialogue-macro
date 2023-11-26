use dialogue_macro::Dialogue;

#[derive(Dialogue)]
enum User {
    Person { name: String, age: u8 },
    Bot { name: String },
}

fn main() {}
