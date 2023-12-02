fn main(){
    let mut p1 = Person{
        name:String::from("John"),
        age:20,
        gender:String::from("Male"),
        hobbies:vec![String::from("Swimming"),String::from("Reading")]
    };

    p1.to_string();
}
use dialoguer::theme::SimpleTheme;

struct Person{
    name:String,
    age:i32,
    gender:String,
    hobbies:Vec<String>
}

impl ToString for Person{
    fn to_string(&self) -> String {
        use dialoguer::Input;
        
        self.name.clone()
    }
}