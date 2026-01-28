use std::{env,fs};

pub fn start_shell_search(){
    println!("{}",env::current_dir().unwrap().display());
    let paths = fs::read_dir(env::current_dir().unwrap()).unwrap();
    for path in paths{
        println!("{}", path.unwrap().path().display());
    }
}
