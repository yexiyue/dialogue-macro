# Introduction

[![GitHub Stars](https://img.shields.io/github/stars/yexiyue/dialogue-macro?style=flat-square)](https://github.com/yexiyue/dialogue-macro) [![Crates.io](https://img.shields.io/crates/v/dialogue-macro?style=flat-square)](https://crates.io/crates/dialogue-macro)

`dialogue-macro` 是一款使用 Rust 语言设计的宏库，致力于无缝对接并增强 `dialoguer` 库的功能表现，采用声明式的编程模式简化命令行提示界面的构建流程。该库在实现结构化、层次化的用户交互体验上表现出色，尤其适应于命令行应用程序中各类复杂信息收集和配置选项设定的需求。

**核心特性概览：**

1. **结构体与枚举支持**：
   dialogue-macro 允许开发者通过派生宏 (`#[derive(Asker)]` 和 `#[derive(EnumAsker)]`) 将自定义结构体及枚举类型转换为功能完备的命令行询问器，从而依据预定义的字段或枚举成员进行高效的数据采集。

2. **嵌套式交互逻辑**：
   提供了对嵌套子询问器的支持，通过在结构体成员上标注 `#[asker(SubAsker)]` 属性，可实现多层次、递进式的用户交互过程，以应对不同场景下的深度信息获取需求。

3. **主题定制能力**：
   支持主题设置，如示例中的 `dialogue_macro::ColorfulTheme`，使得开发者能够根据实际需要选择和应用不同的视觉风格，提升命令行提示界面的用户体验和可读性。



文档中所展示的示例代码片段，可以在[GitHub仓库](https://github.com/yexiyue/dialogue-macro)的examples目录下找到并获取。
