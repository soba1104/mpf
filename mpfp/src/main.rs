use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::Error;
use std::io::Cursor;
use std::io::SeekFrom;
use std::io::ErrorKind;

extern crate byteorder;
use byteorder::{BigEndian, ReadBytesExt};

fn process(path: &String) -> Result<(), Error> {
    println!("process: {:?}.", path);
    let mut buf: [u8; 8] = [0; 8];
    let mut file = File::open(path)?;
    let mut position: u64 = 0;

    loop {
        match file.read_exact(&mut buf) {
            Ok(()) => {
                let mut c = Cursor::new(buf);
                let size = c.read_u32::<BigEndian>()?;
                let mut name = String::new();
                c.read_to_string(&mut name)?;
                println!("read: buf = {:?}, size = {:?}, name = {:?}.", buf, size, name);
                position += size as u64;
                file.seek(SeekFrom::Start(position))?;
            },
            Err(ref err) if err.kind() == ErrorKind::UnexpectedEof => {
                println!("done.");
                break;
            }
            Err(err) => {
                println!("unexpected error: {:?}.", err);
                break;
            }
        }
    }
    // TODO 指定したサイズよりも読み込んだ量が少なかったらエラーを返す。
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path: &String = &args[1];
    let result = process(path);
    println!("result: {:?}.", result);
}
