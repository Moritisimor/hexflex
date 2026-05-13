mod helpers;
mod flags;

use std::fs;

use clap::Parser;
use owo_colors::OwoColorize;

use crate::flags::Flags;

fn main() -> anyhow::Result<()> {
    let flags = Flags::parse();

    let data = fs::read(flags.input_file)?;
    if helpers::is_elf(&data) {
        println!("{}", "This file is probably ELF!".blue())
    }

    let mut idx = 0;
    match flags.save {
        Some(file_name) => {
            let mut buf = String::new();
            for byte in data {
                idx += 1;
                buf += &helpers::format_line(byte, idx);
                buf += "\n"
            }
            
            std::fs::write(&file_name, buf)?;
            println!("{} {}", "Successfully saved content to:".green(), &file_name.magenta());
        }
        
        None => {
            for byte in data {
                idx += 1;
                helpers::print_line(byte, idx);
            }
        }
    }

    Ok(())
}
