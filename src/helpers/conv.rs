use anyhow::bail;

/*
 * Makes a usize of a string, but also allows
 * for hexadecimal and binary representations
*/
pub fn usize_of_str(parsee: &str) -> anyhow::Result<usize> {
    if let Some(x) = parsee.strip_prefix("0x") {
        match usize::from_str_radix(x, 16) {
            Ok(i) => return Ok(i),
            Err(_) => bail!("Error while parsing {parsee} to usize"),
        }
    };

    if let Some(x) = parsee.strip_prefix("0b") {
        match usize::from_str_radix(x, 2) {
            Ok(i) => return Ok(i),
            Err(_) => bail!("Error while parsing {parsee} to usize"),
        }
    };

    match parsee.parse::<usize>() {
        Ok(i) => Ok(i),
        Err(_) => bail!("Error while parsing {parsee} to usize"),
    }
}

// Copy-Paste XD
pub fn u8_of_str(parsee: &str) -> anyhow::Result<u8> {
    if let Some(x) = parsee.strip_prefix("0x") {
        match u8::from_str_radix(x, 16) {
            Ok(i) => return Ok(i),
            Err(_) => bail!("Error while parsing {parsee} to u8"),
        }
    };

    if let Some(x) = parsee.strip_prefix("0b") {
        match u8::from_str_radix(x, 2) {
            Ok(i) => return Ok(i),
            Err(_) => bail!("Error while parsing {parsee} to u8"),
        }
    };

    match parsee.parse::<u8>() {
        Ok(i) => Ok(i),
        Err(_) => bail!("Error while parsing {parsee} to u8"),
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
