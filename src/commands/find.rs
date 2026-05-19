use anyhow::bail;
use owo_colors::OwoColorize;

use crate::helpers;

// Finds the needle in the haystack
fn find(needle: &[u8], haystack: &[u8]) -> anyhow::Result<()> {
    if needle.is_empty() {
        bail!("Cannot search for an empty byte sequence.")
    }

    println!("{}", "Searching...".green());
    let mut matches = 0;
    let mut idx = 0;
    for b in haystack {
        if *b == needle[0] {
            if haystack[idx..].starts_with(needle) {
                println!(
                    "{} {:#010x} - {:#010x}",
                    "Match found at:".green(),
                    idx.blue(),
                    (idx + needle.len() - 1).blue()
                );

                matches += 1;
            }
        }

        idx += 1;
    }

    match matches {
        0 => println!("{}", "No matches for this byte sequence".red()),
        1 => println!("{} {}", 1.blue(), "Match!".green()),
        _ => println!("{} {}", matches.blue(), "Matches!".green()),
    };

    Ok(())
}

pub fn find_string(buf: &[u8], searchee: &str) -> anyhow::Result<()> {
    match searchee.strip_prefix("fs ") {
        Some(s) => Ok(find(s.as_bytes(), &buf)?),
        None => bail!("No input."),
    }
}

pub fn find_bytes(buf: &[u8], args: &[&str]) -> anyhow::Result<()> {
    match helpers::conv::sequence(
        args[1..]
            .iter()
            .map(|c| helpers::conv::u8_of_str(c))
            .collect(),
    ) {
        Some(b) => Ok(find(&b, &buf)?),
        None => bail!("One or more of the bytes you entered could not be parsed.")
    }
}
