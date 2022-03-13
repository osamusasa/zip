use std::fs::File;
use std::io::prelude::*;
use std::io::*;

use super::util::match_from_end;
use super::eocd::Eocd;

pub fn read(url: &str){
    let mut file = File::open(url).unwrap();
    let mut buffer :[u8;30] = [0; 30];
    file.metadata().unwrap().len();
    file.seek(SeekFrom::End(-30));
    file.read_exact(&mut buffer).unwrap();



    println!("{:?}", buffer);

    let buffer_eocd = match_from_end(&buffer, &[0x50, 0x4B, 0x05, 0x06]).unwrap();
    let eocd = Eocd::create(buffer_eocd).unwrap();
    println!("{:?}", eocd);
}