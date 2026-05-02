use std::fs;

use anyhow::bail;
use owo_colors::OwoColorize;

fn is_elf(data: &[u8]) -> bool {
    data.starts_with(&[0x7f, 0x45, 0x4c, 0x46])
}

fn is_probably_printable(c: char) -> bool {
    !c.is_control() && !c.is_whitespace()
}

/*
 * Weird name, but what this function does is detect whether a character
 * is something like a tab, newline, carriage return etc.
*/
fn is_special_control<'a>(c: u8) -> Option<&'a str> {
    match c {
        0x20 => Some("SPACE"),
        0xa => Some("LINEFEED"),
        0xd => Some("CARRIAGE RETURN"),
        0x9 => Some("TAB"),
        _ => None,
    }
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
        print!(
            "{}:\t{:#010b}\t|\t{:#x}\t|\t{}\t",
            idx.blue(),
            byte.magenta(),
            byte.green(),
            byte.yellow()
        );

        if is_probably_printable(byte as char) {
            print!("| ({})", (byte as char).cyan())
        } else {
            if let Some(kind) = is_special_control(byte) {
                print!("| [{}]", kind.cyan());
            }
        }

        println!();
    }

    Ok(())
}
