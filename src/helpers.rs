use owo_colors::OwoColorize;

// Makes a usize of a string, but also allows for hexadecimal representations
pub fn usize_of_str(parsee: &str) -> Option<usize> {
    match parsee.parse::<usize>() {
        Ok(i) => Some(i),
        Err(_) => {
            let actual = match parsee.strip_prefix("0x") {
                Some(a) => a,
                None => parsee,
            };

            match usize::from_str_radix(actual, 16) {
                Ok(i) => Some(i),
                Err(_) => None,
            }
        }
    }
}

pub fn is_elf(data: &[u8]) -> bool {
    data.starts_with(&[0x7f, 0x45, 0x4c, 0x46])
}

pub fn is_probably_printable(c: char) -> bool {
    !c.is_control() && !c.is_whitespace()
}

/*
 * Weird name, but what this function does is detect whether a character
 * is something like a tab, newline, carriage return etc.
*/
pub fn is_special_control<'a>(c: u8) -> Option<&'a str> {
    match c {
        0x20 => Some("SPACE"),
        0xa => Some("LINEFEED"),
        0xd => Some("CARRIAGE RETURN"),
        0x9 => Some("TAB"),
        _ => None,
    }
}

pub fn print_line(byte: u8, idx: usize) {
    print!(
        "{:#010x}:\t{:#010b}\t|\t{:#x}\t|\t{}\t",
        idx.blue(),
        byte.magenta(),
        byte.green(),
        byte.yellow()
    );

    if is_probably_printable(byte as char) {
        print!("| ({})", (byte as char).cyan())
    } else if let Some(kind) = is_special_control(byte) {
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

    if is_probably_printable(byte as char) {
        *buf += &format!("| ({})", (byte as char));
    } else if let Some(kind) = is_special_control(byte) {
        *buf += &format!("| [{}]", kind);
    };

    *buf += "\n";
}
