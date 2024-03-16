/*!

# dialogue-macro

[文档教程](https://yexiyue.github.io/dialogue-macro/)

`dialogue-macro` 是一款使用 Rust 语言设计的宏库，致力于无缝对接并增强 `dialoguer` 库的功能表现，采用声明式的编程模式简化命令行提示界面的构建流程。该库在实现结构化、层次化的用户交互体验上表现出色，尤其适应于命令行应用程序中各类复杂信息收集和配置选项设定的需求。

**核心特性概览：**

1. **结构体与枚举支持**：
   dialogue-macro 允许开发者通过派生宏 (`#[derive(Asker)]` 和 `#[derive(EnumAsker)]`) 将自定义结构体及枚举类型转换为功能完备的命令行询问器，从而依据预定义的字段或枚举成员进行高效的数据采集。

2. **嵌套式交互逻辑**：
   提供了对嵌套子询问器的支持，通过在结构体成员上标注 `#[asker(SubAsker)]` 属性，可实现多层次、递进式的用户交互过程，以应对不同场景下的深度信息获取需求。

3. **主题定制能力**：
   支持主题设置，如示例中的 `dialogue_macro::ColorfulTheme`，使得开发者能够根据实际需要选择和应用不同的视觉风格，提升命令行提示界面的用户体验和可读性。



# 快速开始

首先通过cargo添加`dialogue-macro`依赖:

```bash
cargo add dialogue-macro
```

然后在您的Rust代码中导入该crate:

```rust
use dialogue_macro::Asker;
```

接下来,使用`#[derive(Asker, Debug)]`宏来为需要交互式输入的结构体派生相关trait:

```rust
#[derive(Asker, Debug)]
struct User {
    #[input(prompt = "请输入您的名字:")]
    name: String,
    age: u32,
}
```

`#[derive(Asker)]`会自动为结构体实现`asker()`构造器方法。对于带有`#[input(prompt = "...")]`属性的字段,您可以直接调用同名方法(无需再传入提示文本作为参数)。

最后,使用派生的方法链式调用来获取用户输入:

```rust
fn main() {
    let user = User::asker()
        .name()
        .age("请输入您的年龄:")
        .finish();

    println!("{:?}", user);
}
```

 */

mod theme;
pub use dialogue_core::{Asker, EnumAsker};
#[doc(hidden)]
pub use dialoguer;
#[doc(hidden)]
pub use theme::ColorfulTheme;

/**
# Build Trait

- 构造方法封装： Build trait 需要在实现了它的结构体或枚举中提供 .build() 方法，这个方法会启动并执行所有需要收集用户输入的操作。
- 嵌套与递归： 当结构体或枚举中的某个字段使用了 SubAsker 特性时，表明该字段也是一个实现了 Build trait 的类型。通过调用 .build() 方法，可以递归地收集这些嵌套结构中的信息。
- 简化输入流程： 使用 Build trait 可以避免逐个调用每个字段对应的 .asker() 和 .finish() 方法，而是通过一次 .build() 调用来完成整个对象实例化过程，使得代码更简洁且易于管理。
- 统一接口： 所有作为子询问器使用的结构体都需要实现 Build trait，这保证了不同层次的对象都能遵循一致的构建逻辑。

# Example
```rust
#![allow(unused)]

use dialogue_macro::{Asker, Build, EnumAsker};

#[derive(Debug, Asker)]
#[asker(theme = "dialogue_macro::ColorfulTheme")]
struct UserInput {
    username: String,
    is_student_status: bool,
    educational_institution: String,

    #[asker(SubAsker)]
    detailed_info: DetailedUserInfo,
    #[asker(SubAsker)]
    choice: EnumSelection,
}

#[derive(Debug, Clone, Asker)]
struct DetailedUserInfo {
    email_address: String,
    age: u8,
}

#[derive(Debug, EnumAsker, Clone)]
enum EnumSelection {
    OptionD,
    OptionE,
    OptionF,
}

impl Build for DetailedUserInfo {
    fn build() -> Self {
        Self::asker()
            .email_address("请提供您的邮箱地址")
            .age("请输入您的年龄")
            .finish()
    }
}

fn main() {
    let collected_user_input = UserInput::asker()
        .username("请输入用户名")
        .is_student_status("您是否是学生身份?")
        .educational_institution("请输入您的学校名称")
        .detailed_info()
        .choice()
        .finish();

    println!("{:?}", collected_user_input);
}

```
*/
pub trait Build {
    fn build() -> Self;
}
