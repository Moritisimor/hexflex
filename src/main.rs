use std::fs;

use anyhow::bail;
use owo_colors::OwoColorize;

fn is_elf(data: &[u8]) -> bool {
    data.starts_with(&[0x7f, 0x45, 0x4c, 0x46])
}

fn main() -> anyhow::Result<()> {
    let file_name = match std::env::args().nth(1) {
        Some(f) => f,
        None => bail!("".red()),
    };

    let data = fs::read(file_name)?;
    if is_elf(&data) {
        println!("{}", "This file is probably ELF!".blue())
    }

    let mut idx = 0;
    for byte in data {
        idx += 1;
        println!(
            "{}:\t{:#010b}\t|\t{:#x}\t|\t{}",
            idx.blue(),
            byte.magenta(),
            byte.green(),
            byte.yellow()
        );
    }

    Ok(())
}
