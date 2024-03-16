/*!
该crate是dialogue-macro库的核心库，具体的用法请参考[dialogue-macro](https://docs.rs/dialogue-macro/latest/dialogue_macro/)
 */

use asker::entrypoint;
use proc_macro::TokenStream;
mod asker;
pub(crate) mod dialoguer_list;
mod enum_asker;
mod utils;

/**
# Asker 属性宏

Asker属性宏提供了一种便捷的方式来实现用户交互式输入，支持以下五种属性宏：

1. `input`
2. `confirm`
3. `password`
4. `select`
5. `multiselect`


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
    attributes(input, confirm, password, select, multiselect, asker)
)]
pub fn asker(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as syn::DeriveInput);
    entrypoint(&st)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/**
# EnumAsker

`EnumAsker` 是一个针对枚举类型的交互式询问功能宏，用于在命令行界面中提供用户选择。使用此宏时需要注意，被修饰的枚举变体不能是带有命名字段的结构体类型，而应该采用元组或者无数据变体的形式。

**不支持的枚举定义示例：**

```rust
#[derive(EnumAsker)]
enum Test {
    A {
        name: String,
    },
}
```
上述代码中的 `Test` 枚举包含名为 `A` 的变体，该变体内含一个名为 `name` 的字符串字段。这种带命名字段的结构体风格并不符合 `EnumAsker` 的使用规范。

**支持的枚举定义形式：**

1. **元组列表形式**：

```rust
#[derive(EnumAsker)]
enum Test {
    A(Student, User),
    B(User),
}
```
在这种情况下，`Test` 枚举的变体以元组形式出现，如 `A(Student, User)` 和 `B(User)`，这些变体可以包含其他结构体类型的组合，这完全满足 `EnumAsker` 的使用条件。

2. **空或无数据变体形式**：

```rust
#[derive(EnumAsker)]
enum Test {
    A,
    B,
    C,
}
```
同时，`EnumAsker` 也支持仅包含空或无数据变体（例如 `A`、`B` 和 `C`）的枚举定义。



### 配置选项说明

`EnumAsker` 提供了两种级别的配置项：枚举级别属性和变体级别属性。

**枚举级别属性**：
- **prompt**: 指定用户在进行选择时的提示信息文本。
- **default**: 设置默认选择项，其值应为枚举变体名称。
- **theme**: 设定对话框主题样式，具体主题将在后续部分详细说明。

```rust
#[derive(Debug, EnumAsker)]
#[asker(
    prompt = "选择注册类型",
    default = "Student",
    theme = "dialoguer::theme::ColorfulTheme"
)]
enum Register {
    ...
}
```

**变体级别属性**：

- **label**: 用于配置各个选择项在命令行界面上显示的标签文本。

```rust
#[derive(Debug, EnumAsker)]
#[asker(...)]
enum Register {
    #[asker(label = "注册为普通用户")]
    User(User),
    #[asker(label = "注册为学生")]
    Student(Student),
    #[asker(label = "退出")]
    Exit,
}
```



**注意：**

`EnumAsker`宏不会实现`asker()`方法，而是自动实现`Build` trait，通过调用`build()`方法即可收集用户输入。



### 示例

#### 定义结构体与实现`Build` trait

首先定义了两个结构体：`User` 和 `Student`，分别用于存储普通用户和学生的信息。这两个结构体均派生了 `Asker` 和 `Clone` 特质，并实现了 `Build` trait：

```rust
#[derive(Debug, Asker, Clone)]
struct User {
    #[input(with_default = true)]
    username: String,
    email: String,
}

impl Build for User {
    fn build() -> Self {
        // 用户名提供默认值，邮件无默认值
        Self::asker()
            .username("Enter your username", "default_username".to_string())
            .email("Enter your email")
            .finish()
    }
}

#[derive(Debug, Asker, Clone)]
pub struct Student {
    username: String,
    student_id: String,
}

impl Build for Student {
    fn build() -> Self {
        // 分别询问用户名和学生ID
        Self::asker()
            .username("Enter your username")
            .student_id("Enter your student ID")
            .finish()
    }
}
```

#### 利用`EnumAsker`处理枚举选项

接下来，我们定义了一个名为 `Register` 的枚举类型，它使用了 `EnumAsker` 宏：

```rust
#[derive(Debug, EnumAsker)]
#[asker(
    prompt = "Select registration type",
    default = "Student",
  	theme = "dialoguer::theme::ColorfulTheme"
)]
enum Register {
    // 普通用户注册选项
    #[asker(label = "Register as a regular user")]
    User(User),
    // 学生注册选项
    #[asker(label = "Register as a student")]
    Student(Student),
    // 退出程序的选项
    #[asker(label = "Exit")]
    Exit,
}
```

在这个枚举中，每个变体都关联到一个结构体实例，并通过 `label` 参数指定了命令行界面中的显示文本。此外，我们还设置了 `prompt` 作为主提示信息、`default` 为默认选项。

#### 主函数逻辑

在 `main` 函数中，我们创建并获取用户的选择结果：

```rust
fn main() {
    let choice = Register::build();
    
    match choice {
        Register::User(user) => println!("Registered regular user: {:?}", user),
        Register::Student(student) => println!("Registered student: {:?}", student),
        Register::Exit => println!("Exiting..."),
    }
}
```

当运行程序时，将按照以下流程执行：

1. 根据 `EnumAsker` 设置显示交互式菜单，用户可以选择注册类型（普通用户、学生或退出）。
2. 根据用户的选项，调用相应结构体的 `build()` 方法收集用户信息。
3. 根据匹配结果输出已注册的用户或学生信息，若选择退出则打印退出信息。


 */
#[proc_macro_derive(EnumAsker, attributes(asker))]
pub fn enum_asker(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as syn::DeriveInput);
    enum_asker::enum_asker_build(st)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
