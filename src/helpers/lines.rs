use owo_colors::OwoColorize;
use crate::helpers;

pub fn print_line(byte: u8, idx: usize) {
    print!(
        "{:#010x}:\t{:#010b}\t|\t{:#x}\t|\t{}\t",
        idx.blue(),
        byte.magenta(),
        byte.green(),
        byte.yellow()
    );

    if helpers::metadata::is_probably_printable(byte as char) {
        print!("| ({})", (byte as char).cyan())
    } else if let Some(kind) = helpers::metadata::is_special_control(byte) {
        print!("| [{}]", kind.cyan());
    }

    println!()
}

pub fn format_line(byte: u8, idx: usize, buf: &mut String) {
    buf.clear();

    *buf += &format!(
        "{:#010x}:\t{:#010b}\t|\t{:#04x}\t|\t{}\t",
        idx, byte, byte, byte
    );

    if helpers::metadata::is_probably_printable(byte as char) {
        *buf += &format!("| ({})", (byte as char));
    } else if let Some(kind) = helpers::metadata::is_special_control(byte) {
        *buf += &format!("| [{}]", kind);
    };

    *buf += "\n";
}
