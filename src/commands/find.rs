use owo_colors::OwoColorize;

use crate::helpers;

// Finds the needle in the haystack
fn find(needle: &[u8], haystack: &[u8]) {
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
    }
}

pub fn find_string(buf: &Vec<u8>, args: &Vec<&str>) {
    let search_bytes: Vec<u8> = args[1..].join(" ").chars().map(|c| c as u8).collect();

    if search_bytes.is_empty() {
        println!(
            "{}\n{}",
            "Invalid amount of arguments.".red(),
            "Usage: find <bytes...>".green()
        );

        return;
    }

    find(&search_bytes, &buf)
}

pub fn find_bytes(buf: &Vec<u8>, args: &Vec<&str>) {
    let search_bytes: Vec<_> = match helpers::conv::sequence(
        args[1..]
            .join(" ")
            .split_whitespace()
            .map(|c| helpers::conv::u8_of_str(c))
            .collect(),
    ) {
        Some(b) => b,
        None => {
            println!(
                "{}",
                "One or more of the bytes you entered could not be parsed.".red()
            );
            
            return;
        }
    };

    find(&search_bytes, &buf)
}
