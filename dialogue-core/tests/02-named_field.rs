use dialogue_core::Dialogue;

#[derive(Dialogue)]
enum User {
    Person { name: String, age: u8 },
    Bot { name: String },
}

fn main() {}
