use owo_colors::OwoColorize;

use crate::helpers;

// Finds the needle in the haystack
fn find(needle: &[u8], haystack: &[u8]) {
    if needle.is_empty() {
        println!("{}", "No input.".red());
        return
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
    }
}

pub fn find_string(buf: &Vec<u8>, args: &Vec<&str>) {
    find(&args[1..].join(" ").chars().map(|c| c as u8).collect::<Vec<_>>(), &buf)
}

pub fn find_bytes(buf: &Vec<u8>, args: &Vec<&str>) {
    match helpers::conv::sequence(
        args[1..]
            .join(" ")
            .split_whitespace()
            .map(|c| helpers::conv::u8_of_str(c))
            .collect(),
    ) {
        Some(b) => find(&b, &buf),
        None => {
            println!(
                "{}",
                "One or more of the bytes you entered could not be parsed.".red()
            );
            
            return;
        }
    }
}
