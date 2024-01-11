use dialogue_core::Asker;

#[derive(Asker,Debug)]
struct User {
    #[abc]
    name: String,
    #[input]
    email: String,
    #[input(prompt = "请输入邮箱2")]
    email2: Option<String>,
}

fn main() {
    let mut user = User::asker();
    let user = user.email2().name().email().finish();
    println!("{:?}", user);
}
