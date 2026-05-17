use owo_colors::OwoColorize;
use std::io;

/*
 * Makes a usize of a string, but also allows
 * for hexadecimal and binary representations
*/
pub fn usize_of_str(parsee: &str) -> Option<usize> {
    if let Some(x) = parsee.strip_prefix("0x") {
        match usize::from_str_radix(x, 16) {
            Ok(i) => return Some(i),
            Err(_) => return None,
        }
    };

    if let Some(x) = parsee.strip_prefix("0b") {
        match usize::from_str_radix(x, 2) {
            Ok(i) => return Some(i),
            Err(_) => return None,
        }
    };

    match parsee.parse::<usize>() {
        Ok(i) => Some(i),
        Err(_) => None,
    }
}

// Copy-Paste XD
pub fn u8_of_str(parsee: &str) -> Option<u8> {
    if let Some(x) = parsee.strip_prefix("0x") {
        match u8::from_str_radix(x, 16) {
            Ok(i) => return Some(i),
            Err(_) => return None,
        }
    };

    if let Some(x) = parsee.strip_prefix("0b") {
        match u8::from_str_radix(x, 2) {
            Ok(i) => return Some(i),
            Err(_) => return None,
        }
    };

    match parsee.parse::<u8>() {
        Ok(i) => Some(i),
        Err(_) => None,
    }
}

// sequence :: [Maybe a] -> Maybe [a]
pub fn sequence<T>(list: Vec<Option<T>>) -> Option<Vec<T>> {
    let mut acc: Vec<T> = vec![];
    for t in list {
        match t {
            None => return None,
            Some(i) => acc.push(i),
        }
    }

    Some(acc)
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

pub fn make_error(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, msg)
}

pub fn get_hist_file_path() -> anyhow::Result<String> {
    let mut home_dir = std::env::home_dir()
        .ok_or(make_error("Error while getting home directory"))?
        .to_str()
        .ok_or(make_error("Error while converting home directory to str"))?
        .to_string();

    home_dir.push_str(std::path::MAIN_SEPARATOR_STR);
    home_dir.push_str(".hexflexhist");
    Ok(home_dir)
}
