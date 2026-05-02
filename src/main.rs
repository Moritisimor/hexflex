use std::{fs, io};

fn make_error(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, msg)
}

fn is_elf(data: &[u8]) -> bool {
    data.starts_with(&[0x7f, 0x45, 0x4c, 0x46])
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_name = std::env::args()
        .nth(1)
        .ok_or(make_error("Please choose a file name"))?;

    let data = fs::read(file_name)?;
    if is_elf(&data) {
        println!("This file is probably elf.")
    }
    
    let mut idx = 0;
    for byte in data {
        idx += 1;
        println!("{idx}: {:#010b} | {:#x} | {}", byte, byte, byte); 
    }

    Ok(())
}
