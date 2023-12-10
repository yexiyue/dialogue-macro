use std::vec;

#[allow(unused)]
use dialogue_macro::Dialogue;

#[derive(Dialogue)]
struct User {
    #[dialogue(prompt="请选择你的学校",options=["清华大学","北京大学","上海交通大学"],default="清华大学")]
    school: String,
}

fn main() {
    let school = User::school();
    let mut b = Build {
        name: "".to_string(),
        version: "".to_string(),
        target: "".to_string(),
    };
    b.name("hello").version("world");
    println!("{:?}", b);
    println!("school:{}", school);
    dialoguer::Input::with_theme(&dialogue_macro::ColorfulTheme::default())
        .with_prompt("请输入你的名字")
        .default(123)
        .interact()
        .unwrap();
    dialoguer::MultiSelect::new()
        .items(&["foo", "bar"])
        .defaults(&[true, false])
        .interact()
        .unwrap();
    dialoguer::Password::new()
        .with_prompt("请输入你的密码")
        .with_confirmation("请再次输入你的密码", "秘密错误")
        .interact()
        .unwrap();
    let vec = vec!["1", "2", "3"];
    let d=vec.iter().position(|x| x == &"2").expect("默认值不在选项中");
}

#[derive(Debug)]
struct Build {
    pub name: String,
    pub version: String,
    pub target: String,
}

impl Build {
    pub fn name<T: Into<String>>(&mut self, name: T) -> &mut Self {
        self.name = name.into();
        self
    }
    pub fn version<T: Into<String>>(&mut self, version: T) -> &mut Self {
        self.version = version.into();
        self
    }
}
