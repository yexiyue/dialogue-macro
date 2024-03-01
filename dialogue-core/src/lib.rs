/*!
该crate是dialogue-macro库的核心库，具体的用法请参考[dialogue-macro](https://docs.rs/dialogue-macro/latest/dialogue_macro/)
 */

use dialoguer::entrypoint;
use proc_macro::TokenStream;
mod dialoguer;
pub(crate) mod dialoguer_list;
mod utils;
pub(crate) static mut DIALOGUE_THEME: i32 = 1;

/**
# Asker 属性宏

Asker属性宏提供了一种便捷的方式来实现用户交互式输入，支持以下五种属性宏：

1. `input`
2. `confirm`
3. `password`
4. `select`
5. `multiselect`

此外，还支持通过theme宏来定制主题样式，包括：

 - simple：简洁主题
 - colorful：多彩主题
 - colorful_macro：多彩主题（针对multiselect做了特别样式调整）


### Input
`input`属性宏用于创建输入框。可以通过`prompt`属性设置提示信息，并通过`default`属性设置默认值。

**示例**

```rust
#![allow(unused)]
use dialogue_macro::Asker;

#[derive(Asker, Debug)]
struct User {
    name: String,
    #[input(prompt = "请输入您的邮箱地址: ")]
    email: String,
    #[input(default = 16)]
    age: u8,
    #[input(prompt = "请输入您的学校: ", default = "清华大学")]
    school: Option<String>,
}

fn main() {
    let user = User::asker()
        .name("请输入您的姓名: ")
        .email()
        .age("请输入您的年龄: ")
        .school()
        .finish();

    println!("{:?}", user);
}
```

### Confirm
`confirm`属性宏用于创建确认对话框，允许用户选择是/否。通过`prompt`属性设置询问内容，也可以通过`default`属性预设默认值。

**示例**

```rust
#![allow(unused)]
use dialogue_macro::Asker;

#[derive(Asker, Debug)]
struct User {
    boy: bool,
    #[confirm(prompt = "您是否是学生?")]
    student: Option<bool>,
}

fn main() {
    let user = User::asker().boy("您是男孩吗？").student().finish();
    println!("{:?}", user);
}
```
### Password
`password`属性宏用于创建密码输入框，可以设置`prompt`属性来定义初始密码输入提示，`confirmation`属性用来设置再次确认密码的提示，以及在两次输入不一致时显示的`mismatch`提示信息。

**示例**

```rust
#![allow(unused)]
use dialogue_macro::Asker;

#[derive(Asker, Debug)]
struct User {
    #[password()]
    password: Option<String>,
    #[password(
        prompt = "请输入您的密码:",
        confirmation = "请再次输入您的密码:",
        mismatch = "您的密码不匹配"
    )]
    password2: String,
}

fn main() {
    let user = User::asker().password2().password("请输入您的密码:").finish();
    println!("{:?}", user);
}
```
### Select
`select`属性宏用于创建下拉选择框，通过`prompt`属性设置提示语，`options`属性定义可选项列表，并可通过`default`属性设置默认选中项。

**示例**

```rust
#![allow(unused)]
use dialogue_macro::Asker;

#[derive(Asker, Debug)]
struct User {
    #[select(prompt = "请选择您的性别", options = ["男", "女", "其他"], default = 1)]
    sex: String,
    #[select(prompt = "请选择您的学校: ", default = 1,options=[
        School{
            name: "清华大学".to_string(),
        },
        School{
            name: "北京大学".to_string(),
        },
        School{
            name: "河南理工大学".to_string(),
        }
    ])]
    school: Option<School>,
}

#[derive(Debug, Clone)]
struct School {
    name: String,
}

impl ToString for School {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

fn main() {
    let user = User::asker().sex().school().finish();

    println!("{:?}", user);
}
```
### MultiSelect
`multiselect`属性宏用于创建多选框，用户可以从多个选项中进行选择。同样通过`prompt`属性设置提示语，`options`属性定义可选项列表，`default`属性设置默认已选中的项。

**示例**

```rust
#![allow(unused)]
use dialogue_macro::Asker;

#[derive(Asker, Debug)]
struct User {
    #[multiselect(prompt = "请选择您的兴趣爱好", options = ["吃", "睡", "编程"], default=[1])]
    favorite: Vec<String>,
    #[multiselect(prompt = "请选择您的学校: ", default = [1,2],options=[
        School{
            name: "清华大学".to_string(),
        },
        School{
            name: "北京大学".to_string(),
        },
        School{
            name: "河南理工大学".to_string(),
        }
    ])]
    school: Vec<School>,
}

#[derive(Debug, Clone)]
struct School {
    name: String,
}

impl ToString for School {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

fn main() {
    let user = User::asker().school().favorite().finish();

    println!("{:?}", user);
}
```
 */

#[proc_macro_derive(
    Asker,
    attributes(input, confirm, password, select, multiselect, theme, asker)
)]
pub fn dialoguer(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as syn::DeriveInput);
    entrypoint(&st)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
