[![GitHub Stars](https://img.shields.io/github/stars/yexiyue/dialogue-macro?style=flat-square)](https://github.com/yexiyue/dialogue-macro) [![Crates.io](https://img.shields.io/crates/v/dialogue-macro?style=flat-square)](https://crates.io/crates/dialogue-macro) [文档教程](https://yexiyue.github.io/dialogue-macro/)

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


