use std::{env, fs};
use std::fs::File;
use utils::cursor_wrapper::CursorWrapper;
use crate::parser::parse_file;

mod parser;
mod VM;
mod utils;

fn main()
{
    println!("Hello, world!");

    let mut argv : Vec<String> = env::args().collect();

    println!("argc = {}", argv.len());

    if argv.len() < 2 {
        panic!("Arggg pas de param");
    }

    println!("println {}", argv[1]);
    if !fs::exists(&argv[1]).unwrap() {
        panic!("wesh le file existe meme pas");
    }

    let buff = fs::read(&argv[1]).unwrap();
    let mut reader = CursorWrapper::new(buff);

    parse_file(&mut reader).unwrap();
}
