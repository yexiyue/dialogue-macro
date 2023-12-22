mod dialogue;
use define::generate_struct;
use proc_macro::TokenStream;
mod define;

pub(crate) static mut DIALOGUE_THEME: i32 = 1;

/**
 Dialogue derive macro

 # Examples
 ```rust
#[derive(Dialogue)]
 struct User {
     #[dialogue(prompt = "请输入用户名")]
     username: String,
     #[dialogue(password = true, prompt = "请输入密码")]
     password: String,
 }
 ```
 */
#[proc_macro_derive(Dialogue, attributes(dialogue, theme))]
pub fn dialogue_derive(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as syn::DeriveInput);
    dialogue::dialogue_derive(&st)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/**
 Dialogue define macro

 # Examples
 ```rust
 dialogue_define!({
    username=>{
        prompt:"请输入用户名",
        default:"admin"
    },
    password=>{
        ty:"password",
        prompt:"请输入密码",
        confirmation:"请再次输入密码!",
        mismatch:"两次输入的密码不一致",
    }
});
 ```
*/
#[proc_macro]
pub fn dialogue_define(input: TokenStream) -> TokenStream {
    let dialogue_list = syn::parse_macro_input!(input as define::DialogueList);
    generate_struct::generate_struct(&dialogue_list)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
