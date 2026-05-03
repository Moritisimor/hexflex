mod helpers;

use std::fs;

use anyhow::bail;
use owo_colors::OwoColorize;

fn main() -> anyhow::Result<()> {
    let file_name = match std::env::args().nth(1) {
        Some(f) => f,
        None => bail!("Please select a file!".red()),
    };

    let data = fs::read(file_name)?;
    if helpers::is_elf(&data) {
        println!("{}", "This file is probably ELF!".blue())
    }

    let mut idx = 0;
    for byte in data {
        idx += 1;
        print!(
            "{}:\t{:#010b}\t|\t{:#x}\t|\t{}\t",
            idx.blue(),
            byte.magenta(),
            byte.green(),
            byte.yellow()
        );

        if helpers::is_probably_printable(byte as char) {
            print!("| ({})", (byte as char).cyan())
        } else if let Some(kind) = helpers::is_special_control(byte) {
            print!("| [{}]", kind.cyan());
        }

        println!();
    }

    Ok(())
}
