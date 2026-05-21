use crate::helpers;
use owo_colors::OwoColorize;

pub fn print_line(byte: u8, idx: usize) {
    print!(
        "{:#010x}:\t{:#010b}\t|\t{:#x}\t|\t{}\t",
        idx.blue(),
        byte.magenta(),
        byte.green(),
        byte.yellow()
    );

    if helpers::info::is_probably_printable(byte as char) {
        print!("| ({})", (byte as char).cyan())
    } else if let Some(kind) = helpers::info::is_special_control(byte) {
        print!("| [{}]", kind.cyan());
    }

    println!()
}

pub fn format_bytes(bytes: &[u8], buf: &mut String) {
    buf.clear();
    let mut idx = 0;

    for word in bytes.chunks(8) {
        *buf += &format!("{:#010x}: ", idx);
        for byte in word {
            *buf += &format!("{:#04x} ", byte);
        }

        *buf += "|";
        for byte in word {
            if helpers::info::is_probably_printable(*byte as char) {
                buf.push(*byte as char);
            } else {
                buf.push('.')
            }
        }

        *buf += "|";
        idx += 8;
        *buf += "\n";
    }
}
