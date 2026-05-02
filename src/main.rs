use std::{fs, io};

fn make_error(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, msg)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_name = std::env::args()
        .nth(1)
        .ok_or(make_error("Please choose a file name"))?;

    let data = fs::read(file_name)?;
    let mut idx = 0;
    for byte in data {
        idx += 1;
        println!("{idx}: {:#010b} | {:#x} | {}", byte, byte, byte); 
    }

    Ok(())
}
