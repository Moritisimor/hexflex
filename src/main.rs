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
            let mut file_buf = String::new();
            let mut line_buf = String::new();
            
            for byte in data {
                helpers::format_line(byte, idx, &mut line_buf);
                file_buf += &line_buf;
                idx += 1;
            }
            
            std::fs::write(&file_name, file_buf)?;
            println!("{} {}", "Successfully saved content to:".green(), &file_name.magenta());
        }
        
        None => {
            for byte in data {
                helpers::print_line(byte, idx);
                idx += 1;
            }
        }
    }

    Ok(())
}
