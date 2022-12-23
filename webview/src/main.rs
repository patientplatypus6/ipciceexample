

use std::env;

fn main() {
    #![allow(warnings, unused)]
    println!("Inside Webview Cargo");
    let args: Vec<String> = env::args().collect();
    println!("value of args {:?}", args);
}
