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
