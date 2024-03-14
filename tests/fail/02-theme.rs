use dialogue_core::Asker;
#[derive(Asker)]
#[asker(theme = "abc")]
struct User{
    name: String,
    age: u8,
}

fn main() {}
