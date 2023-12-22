# dialogue-macro
一个使用宏方式使用[dialoguer](https://docs.rs/dialoguer/latest/dialoguer/)

### 简介

dialogue-macro使用宏封装了一些常用dialoguer使用方式，支持[dialoguer](https://docs.rs/dialoguer/latest/dialoguer/)以下功能。

- [Password](https://docs.rs/dialoguer/latest/dialoguer/struct.Password.html)
- [Select](https://docs.rs/dialoguer/latest/dialoguer/struct.Select.html)
- [MultiSelect](https://docs.rs/dialoguer/latest/dialoguer/struct.MultiSelect.html)
- [Confirm](https://docs.rs/dialoguer/latest/dialoguer/struct.Confirm.html)
- [Input](https://docs.rs/dialoguer/latest/dialoguer/struct.Input.html)

基于不同的场景，可以分别使用`Dialogue`derive宏，和`dialogue_define`函数式宏，derive宏功能比较简单，定义的时候只支持字面量形式，函数式宏不仅支持字面量，还支持变量的形式。



### Dialogue

使用该derive宏，会根据字段的类型，推断其对话形式，并生成同名方法。

公共字段prompt，如果定义了prompt则调用方法时不需要再传递prompt参数。

#### Example

**具体用法可以参考下面例子**

##### **Input**

```rust
use dialogue_macro::ColorfulTheme;
#[allow(unused)]
use dialogue_macro::Dialogue;
#[derive(Dialogue, Debug)]
struct User {
    name: String,
    age: u32,
    #[dialogue(prompt = "请输入你的邮箱")]
    email: String,
    #[dialogue(prompt = "请输入你的年级", default = "5")]
    grade: i32,
}

fn main() {
    let name = User::name("请输入你的名字");
    let age = User::age("请输入你的年龄");
    let email = User::email();
    let grade = User::grade();
    let user = User {
        name,
        age,
        email,
        grade,
    };

    println!("{:#?}", user);
}

```

##### **Password**

注意：不支持default，启用时必须设置password为true。

```rust
use dialogue_macro::Dialogue;

#[derive(Dialogue, Debug)]
struct Password {
    #[dialogue(password = true)]
    password: String,
    #[dialogue(prompt = "请输入密码", password = true)]
    password2: String,
    #[dialogue(
        prompt = "请输入密码",
        password = true,
        confirmation = "请再次输入密码"
    )]
    password3: String,
    #[dialogue(
        prompt = "请输入密码",
        password = true,
        confirmation = "请再次输入密码",
        mismatch = "两次输入的密码不一致"
    )]
    password4: String,
}

fn main() {
    let p1 = Password::password("请输入密码");
    let p2 = Password::password2();
    let p3 = Password::password3();
    let p4 = Password::password4();
    let password = Password {
        password: p1,
        password2: p2,
        password3: p3,
        password4: p4,
    };
    println!("{:?}", password);
}

```

##### Confirm

```rust
use dialogue_macro::Dialogue;
#[derive(Debug, Dialogue)]
struct Person {
    student:bool,
    #[dialogue(prompt = "你是老师吗",default=false)]
    teacher:bool,
}
fn main() {
    let student=Person::student("你是学生吗？");
    let teacher=Person::teacher();
    let p1 = Person{
        student,
        teacher,
    };
    println!("{:?}", p1);
}
```

##### Select

**可以通过theme属性切换主题。**

**必须指定options才能开启select。**

```rust
use dialogue_macro::Dialogue;

#[derive(Dialogue)]
#[theme(simple)]
struct User {
    #[dialogue(prompt="请选择你的学校",options=["清华大学","北京大学","上海交通大学"],default="上海交通大学")]
    school: String,
}

fn main(){
    let school=User::school();
    println!("school:{}",school);
}
```

##### MultiSelect

**注意：字段类型得是一个数组，且设置options字段才能启用multiselect，default字段必须得是数组形式，不然默认值不生效。**

```rust
#![allow(unused)]

use dialogue_macro::Dialogue;
#[derive(Dialogue)]
struct User {
    #[dialogue(prompt="请选择你意向的学校",options=["清华大学","北京大学","上海交通大学"],default=["清华大学","北京大学","上海交通大学"])]
    school: Vec<String>,
}

fn main() {
    let school = User::school();
    println!("school:{:?}", school);
}

```



### dialogue_define

函数式宏，通过自定义语法，配置dialoguer相关对话。

具有以下这些配置

- default：用于指定默认值，如果是ty是multiselect请使用数组
- confirmation：当ty是password才生效，用于指定第二次输入密码的提示
- mismatch：当ty是password才生效，用于指定两次密码不匹配时的错误提示
- prompt：提示信息
- options：当ty是select或者multiselect才生效，用于指定选择列表
- ty字段指定使用那种dialoguer形式，支持以下值。
  - input
  - select
  - multiselect
  - confirm
  - password

**使用该宏，会在当前作用域自动生成`Dialogue`结构体，具体用法请，参考下面example。**



#### Example

**具体用法可以参考下面例子**

##### **Input**

```rust
#![allow(unused)]
use dialogue_macro::dialogue_define;

static DEFAULT_AGE: i32 = 17;
static PROMPT: &str = "你的名字是什么";
dialogue_define!({
    name=>{
        ty:"input",
        prompt: PROMPT,
        default: "John"
    },
    age<i32>=>{
        default: DEFAULT_AGE
    }
});

fn main() {
    let mut user = Dialogue::new();
    user.name();
    user.age("你的年龄");

    println!("{:#?}", user);
}

```

##### **Password**

注意：不支持default，启用时必须设置password为true。

```rust
#![allow(unused)]
use dialogue_macro::dialogue_define;
static PROMPT: &str = "请输入密码";
dialogue_define!({
    password1=>{
        ty:"password",
    },
    password2=>{
        ty:"password",
        prompt:PROMPT,
    },
    password3=>{
        ty:"password",
        prompt:PROMPT,
        confirmation:"请再次输入密码!",
    },
    password4=>{
        ty:"password",
        prompt:PROMPT,
        confirmation:"请再次输入密码#",
        mismatch:"两次输入的密码不一致#",
    }
});

fn main() {
    let mut password = Dialogue::new();
    password.password1("请输入密码呀");
    password.password2();
    password.password3();
    password.password4();
    println!("{:?}", password);
}

```

##### Confirm

```rust
#![allow(unused)]
use dialogue_macro::dialogue_define;
// 使用变量的形式定义参数
static PROMPT: &str = "你是小学生吗？";
static DEFAULT: bool = true;

// 自动生成一个Dialogue结构体，并实现相关的方法
dialogue_define!({
    student=>{
        ty:"confirm",
        prompt: PROMPT,
        default: DEFAULT,
    },
    teacher=>{
        prompt: "你是老师吗？",
        default: "true",
        ty:"confirm"
    }
});
// 使用别名
type User = Dialogue;

fn main() {
    let mut p1 = User::new();
    p1.student();
    p1.teacher();
    println!("{:?}", p1);
}

```

##### Select

**可以通过theme属性切换主题。**

```rust
#![allow(unused)]
use dialogue_macro::dialogue_define;

#[derive(Debug, Clone)]
struct Person {
    name: &'static str,
    age: i32,
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl ToString for Person {
    fn to_string(&self) -> String {
        self.name.to_string()
    }
}

static PERSONS: [Person; 3] = [
    Person {
        name: "张三",
        age: 18,
    },
    Person {
        name: "李四",
        age: 19,
    },
    Person {
        name: "王五",
        age: 20,
    },
];

static DEFAULT_PERSON: Person = Person {
    name: "李四",
    age: 19,
};

dialogue_define!({
    persons<Person>=>{
        ty:"select",
        options:PERSONS,
        default:DEFAULT_PERSON,
    }
});
fn main() {
    let mut person = Dialogue::new();
    person.persons("请选择人物");
    println!("person:{:#?}", person);
}

```

##### MultiSelect

```rust
#![allow(unused)]
use dialogue_macro::dialogue_define;
use std::any::Any;
#[derive(Debug, Clone)]
struct Person {
    name: &'static str,
    age: i32,
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl ToString for Person {
    fn to_string(&self) -> String {
        self.name.to_string()
    }
}

static PERSONS: [Person; 3] = [
    Person {
        name: "张三",
        age: 18,
    },
    Person {
        name: "李四",
        age: 19,
    },
    Person {
        name: "王五",
        age: 20,
    },
];

static DEFAULT_PERSON: [Person; 2] = [
    Person {
        name: "李四",
        age: 19,
    },
    Person {
        name: "王五",
        age: 20,
    },
];

dialogue_define!({
    persons<Person>=>{
        ty:"multiselect",
        options:PERSONS,
        default:DEFAULT_PERSON,
    }
});
fn main() {
    let mut person = Dialogue::new();
    person.persons("请选择人物");
    println!("person:{:#?}", person);
}

```

