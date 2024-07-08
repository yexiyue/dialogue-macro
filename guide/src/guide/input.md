# Input

`Input`用于接收用户的文本或数字输入。

#### 定义Input字段

使用`dialogue_macro::Asker`宏为结构体派生trait时，框架会自动将以下字段类型识别为`Input`输入字段:

- `String`
- 基本数字类型，如`i32`、`u32`、`usize`等

如果某个字段为非必填项，可将其声明为`Option<String>`或对应的`Option<数字类型>`。这样在调用`finish()`方法时，如果没有调用该同名方法，程序不会panic。

#### 配置选项

- **prompt**: 指定提示用户输入信息的文本内容。
- **default**: 设置字段的默认值，在询问用户时显示该默认值供参考或直接采用。
- **with_default**: 布尔值参数，当设为`true`时，在调用相应方法时需要传递默认值。
- **validate_with**: 指定验证函数，验证用户输入的值是否符合要求。

#### 示例

在以下的 Rust 代码示例中，我们利用 `dialogue_macro` 库来实现一个用户交互式信息收集的场景。定义了一个结构体 `User`，它包含四个字段：`name`、`email`、`age` 和 `school`，分别用于收集用户的姓名、电子邮件地址、年龄和学校信息。

```rust
#![allow(unused)]
use dialogue_macro::Asker;

#[derive(Asker, Debug)]
struct User {
    name: Option<String>,
    #[input(prompt = "请输入您的邮箱地址: ", with_default = true)]
    email: String,
    #[input(with_default = true)]
    age: u8,
    #[input(
        prompt = "请输入您的学校名称: ",
        with_default = true,
        validate_with = |input| {
            if input.contains("大学") { Ok(()) } else { Err("学校名称必须包含大学") }
        }
    )]
    school: Option<String>,
}

fn main() {
    let user = User::asker()
        .name("请输入您的姓名: ")
        .email("yexiyue666@qq.com")
        .age("请输入您的年龄", 17)
        .school("北京大学")
        .finish();

    println!("{:?}", user);
}

```

![image-20240316141307176](input.assets/image-20240316141307176.png)

在上述示例中：

- `name` 字段是一个可选字符串类型，用户可以选择是否输入姓名。
- `email` 字段要求用户输入电子邮件地址，并支持传入默认值，通过 `.email()` 方法传递。
- `age` 字段用于获取用户年龄，也支持默认值，此处默认值为17岁。
- `school` 字段允许用户输入学校名称，同时提供了默认值"清华大学"。

在 `main` 函数中：

- 我们初始化了 `User` 结构体的询问器实例，并为每个字段指定了相应的提示信息和/或默认值。
- 当运行程序时，将按照定义顺序逐个展示各个字段的提示信息，并依据是否设置了默认值进行交互式信息收集。
- 收集完成后，`user.finish()` 返回一个包含了用户输入信息的 `User` 结构体实例，并将其输出至控制台。
