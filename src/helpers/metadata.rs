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
