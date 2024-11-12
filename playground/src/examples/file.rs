use std::{
    fs::File,
    io::{Read, Seek},
};

pub fn file_io() {
    let mut f = File::open("./README.md").unwrap();
    f.seek(std::io::SeekFrom::Start(10)).unwrap();
    let mut buf = [0u8; 10];
    f.read(&mut buf).unwrap();
    let s: String = buf.iter().map(|&n| char::from(n)).collect();
    println!("{:?}", s);
}
