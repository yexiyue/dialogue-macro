use dialogue_macro::Dialogue;
// use wasm_startup::init;

#[derive(Dialogue)]
struct User {
    #[dialogue(default = true, prompt = "是否是管理员")]
    admin: bool,
    #[dialogue(prompt = "请输入地址", default = "四川省达州市")]
    address: String,
    #[dialogue(options=["张三", "李四", "王五"],default="李四")]
    names: String,
    #[dialogue(options=["张三", "李四", "王五"],default=["李四","王五"])]
    names2: Vec<String>,
    #[dialogue(
        prompt = "请输入密码",
        password = true,
        confirmation = "请再次输入密码"
    )]
    password: String,
}

fn main() {
    // User::admin();
    // User::address();
    // User::names("请选择组长");
    // User::names2("请选择组长2");
    // User::password();
}
