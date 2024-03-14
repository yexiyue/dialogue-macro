/*!

# dialogue-macro

[![GitHub Stars](https://img.shields.io/github/stars/yexiyue/dialogue-macro?style=flat-square)](https://github.com/yexiyue/dialogue-macro) [![Crates.io](https://img.shields.io/crates/v/dialogue-macro?style=flat-square)](https://crates.io/crates/dialogue-macro)

dialogue-macro 是一个 Rust 宏封装库，它作为 [dialoguer](https://docs.rs/dialoguer/latest/dialoguer/) 的扩展，为命令行交互界面的构建提供了更为简洁和直观的方法。通过利用 `Asker` 宏的力量，开发者可以迅速搭建一系列用户友好且功能丰富的对话界面。

### 简介

**dialogue-macro** 是一个 Rust 库，它通过宏的方式简化了对 <a href="https://docs.rs/dialoguer/latest/dialoguer/" target="_blank">dialoguer</a> 库的常用对话界面组件的使用，目前支持以下功能：

- [Password](https://docs.rs/dialoguer/latest/dialoguer/struct.Password.html)：密码输入
- [Select](https://docs.rs/dialoguer/latest/dialoguer/struct.Select.html)：单选项选择
- [MultiSelect](https://docs.rs/dialoguer/latest/dialoguer/struct.MultiSelect.html)：多选项选择
- [Confirm](https://docs.rs/dialoguer/latest/dialoguer/struct.Confirm.html)：确认提示
- [Input](https://docs.rs/dialoguer/latest/dialoguer/struct.Input.html)：普通文本输入

### Asker 宏

`Asker` 宏允许您定义一个结构体，并根据结构体中字段的类型自动生成对应类型的对话式用户输入方法。公共字段中的 `prompt` 注解可用于预设提示信息，在调用方法时若已设置则无需额外传递 prompt 参数。

`Asker`宏具有智能类型推断功能，当未明确指定交互类型时，它会根据结构体字段的原始类型自动确定合适的对话交互方式：

1. 当字段为 `bool` 或 `Option<bool>` 类型时，宏会将其识别并转化为 confirm 类型的用户确认提示。
2. 若字段为 `Vec<T>` 类型，则宏将推断其为 multiselect 类型，以便进行多项选择操作。
3. 对于所有其他非上述特殊类型的字段，默认处理方式是将其视为普通的文本输入，即 input 类型。

`Asker`宏借鉴了 Builder 模式的设计理念，使对话流程的构建更加直观和灵活。

有关该宏的详细用法请参考[`Asker`]，下面是一个简单的例子。

### 示例

```rust
#![allow(unused)]
use dialogue_core::Asker;

#[derive(Asker, Debug)]
#[theme(colorful)]
struct User {
    // 用户名输入
    #[input(prompt = "Enter your name:")]
    name: String,

    // 年龄输入，可通过方法调用指定 prompt
    age: u32,

    // 密码输入
    #[password(prompt = "Enter your password:")]
    password: String,

    // 可选邮箱输入
    email: Option<String>,

    // 单项选择
    #[select(prompt="Please select your sex", options = ["Male", "Female", "Other"], default = 1)]
    sex: String,

    // 确认提示
    #[confirm(prompt = "Are you sure?")]
    sure: bool,

    // 多项选择
    #[multiselect(prompt = "Please select your favorite", default=[1])]
    favorite: Vec<String>,
}

fn main() {
    let favorite_options = vec!["eat".to_string(), "sleep".to_string(), "code".to_string()];

    // 使用自动生成的方法构建用户输入流程
    let user = User::asker()
        .name()
        .age("Enter your age:")
        .email("Enter your email:")
        .sex()
        .favorite(&favorite_options)
        .sure()
        .finish();

    println!("{:#?}", user);
}

```


 */

mod theme;
pub use dialogue_core::{Asker, EnumAsker};
#[doc(hidden)]
pub use dialoguer;
#[doc(hidden)]
pub use theme::ColorfulTheme;

pub trait Build {
    fn build() -> Self;
}
