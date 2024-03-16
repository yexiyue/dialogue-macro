Confirm
=========

`Confirm` 用于获取用户的布尔值确认输入(是或否)。

#### **定义 Confirm 字段**

要将字段定义为 `Confirm` 类型,只需将其声明为 `bool` 或 `Option<bool>` 类型。与 `Input` 类似,如果字段为非必填项,可以声明为 `Option<bool>`。



#### **配置选项**

- **prompt**: 指定提示用户进行确认的文本内容。
- **default**: 设置字段的默认值(`true` 或 `false`),在询问用户时显示该默认值供参考或直接采用。
- **with_default**: 布尔值参数,当设为 `true` 时,在调用相应方法时需要传递默认值。



#### **示例**

```rust
#![allow(unused)]
use dialogue_macro::Asker;

#[derive(Asker, Debug)]
struct User {
    boy: bool,

    #[confirm(prompt = "Are you a student?", with_default = true)]
    student: Option<bool>,
}

fn main() {

    let user = User::asker()
        .boy("Are you a boy?")
        .student(true)
        .finish();

    println!("{:?}", user);
}

```

![image-20240316141820068](confirm.assets/image-20240316141820068.png)

在上述代码中：

- `User` 结构体中的 `boy` 字段是一个布尔类型变量，通过询问用户来确定其性别。
- `student` 字段是一个具有默认值的可选布尔类型变量，通过 `#[confirm]` 宏提供了自定义确认提示 "Are you a student?"，默认情况下用户被假设为学生（值为 `true`）。

在 `main` 函数中：

- 我们初始化了 `User` 结构体的询问器实例，并分别为 `boy` 和 `student` 字段指定了提示信息和默认值。
- 当运行程序时，会依次显示预设的提示信息，并根据默认值进行交互式信息收集。
- 收集过程完成后，`.finish()` 方法返回一个包含了用户回答信息的 `User` 结构体实例，并将其输出至控制台。

