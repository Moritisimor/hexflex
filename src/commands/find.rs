use owo_colors::OwoColorize;

pub fn find(buf: &Vec<u8>, args: &Vec<&str>) {
    let search_bytes: Vec<u8> = args[1..].join(" ").chars().map(|c| c as u8).collect();

    if search_bytes.is_empty() {
        println!(
            "{}\n{}",
            "Invalid amount of arguments.".red(),
            "Usage: find <bytes...>".green()
        );

        return;
    }

    println!("{}", "Searching...".green());
    let mut matches = 0;
    let mut idx = 0;
    for b in buf {
        if *b == search_bytes[0] {
            if buf[idx..].starts_with(&search_bytes) {
                println!("{} {:#010x}", "Match found at:".green(), idx.blue());
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
