use std::{
    fs::File,
    io::{Read, Seek, Write},
};

pub fn file_io() {
    let mut f = File::open("./README.md").unwrap();

    let initial_pos = f.stream_position().unwrap();
    println!("initial pos: {}", initial_pos);

    let mut buf = [0u8; 10];
    f.read(&mut buf).unwrap();

    let s = String::from_utf8(buf.into()).unwrap();
    println!("{:?}", s);
}
