#[macro_use]
extern crate stdweb;

pub fn hello() {
    js!(console.log("Hello!"));
}