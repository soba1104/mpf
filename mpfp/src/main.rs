use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::Error;

fn process(path: &String) -> Result<(), Error> {
    println!("process: {:?}.", path);
    let mut buf: [u8; 4] = [0; 4];
    let mut file = File::open(path)?;
    file.read_exact(&mut buf)?;
    // TODO 指定したサイズよりも読み込んだ量が少なかったらエラーを返す。
    println!("head: {:?}.", buf);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path: &String = &args[1];
    let result = process(path);
    println!("result: {:?}.", result);
}
