mod helpers;

use std::fs;

use anyhow::bail;
use owo_colors::OwoColorize;

fn main() -> anyhow::Result<()> {
    let file_name = match std::env::args().nth(1) {
        Some(f) => f,
        None => bail!("Please select a file!"),
    };

    let data = fs::read(file_name)?;
    if helpers::is_elf(&data) {
        println!("{}", "This file is probably ELF!".blue())
    }

    let mut idx = 0;
    for byte in data {
        idx += 1;
        helpers::print_line(byte, idx);
    }

    Ok(())
}
