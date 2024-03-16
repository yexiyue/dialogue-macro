Multiselect
=========

`Multiselect` 用于从给定选项列表中让用户选择多项。

#### 定义 Multiselect 字段

要将字段定义为 `Multiselect` 类型,需要使用 `#[multiselect(...)]` 属性对字段进行标注,字段类型必须为 `Vec<T>`(其中 `T` 实现了 `ToString` trait)。

**注意:** 

- **`Multiselect` 不支持 `Option<Vec<T>>` 类型。**
- **`T`类型也必须实现`Clone`trait**



#### 配置选项

- **prompt**: 指定提示用户进行多选的文本内容。
- **options**: 包含选项列表的数组或向量,每个选项必须实现 `ToString` trait。
- **default**: 设置默认选中项的索引列表(从 0 开始)。
- **with_default**: 布尔值参数,当设为 `true` 时,在调用相应方法时需要传递一个布尔值数组,其中 `true` 表示选中对应索引的选项。传入的布尔值数组长度必须与选项列表长度相同。



#### 示例

```rust
#![allow(unused)]
use dialogue_macro::Asker;

#[derive(Asker, Debug)]
struct User {
    #[multiselect(prompt = "Please select your favorite", options = ["Eat", "Sleep", "Code"], default = [1])]
    favorite: Vec<String>,

    #[multiselect(
        prompt = "Please select your school: ",
        with_default = true,
        options = [
            School { name: "Tsinghua University".to_string() },
            School { name: "Peking University".to_string() },
            School { name: "Henan Polytechnic University".to_string() }
        ]
    )]
    school: Vec<School>,
}

// 定义表示学校的结构体，并实现Debug和Clone特质
#[derive(Debug, Clone)]
struct School {
    name: String,
}

// 实现ToString trait，将School结构体转换为字符串形式以便于展示
impl ToString for School {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

fn main() {

    let user = User::asker()
        .school(&[true, false, true])
        .favorite()
        .finish();


    println!("{:?}", user);
}
```

<img src="multiselect.assets/image-20240316144222151.png" alt="image-20240316144222151"  />

![image-20240316144238610](multiselect.assets/image-20240316144238610.png)

在这个示例中：

- `favorite` 字段是一个多选类型，提供三个预设选项，默认选择了“Sleep”。
- `school` 字段也是一个多选类型，但其选项是由自定义的 `School` 结构体实例表示的，且支持默认选择。通过传入一个布尔值数组来指定哪些学校是默认被选中的，数组长度必须与提供的学校选项列表长度相同。

在 `main` 函数中：

- 首先初始化了 `User` 结构体的询问器实例，并分别为 `school` 和 `favorite` 字段指定了默认选择或直接使用默认选项。
- 当运行程序时，会按照顺序显示预设的提示信息，并根据设置的默认值进行交互式信息收集。
- 收集完成后，`.finish()` 方法返回一个包含了用户选择信息的 `User` 结构体实例，并将其输出至控制台。
