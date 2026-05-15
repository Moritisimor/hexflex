use crate::helpers;

use owo_colors::OwoColorize;

pub fn read_byte(buf: &Vec<u8>, args: &Vec<&str>) {
    let start_idx = match args.get(1) {
        Some(i) => match helpers::usize_of_str(i) {
            Some(n) => n,
            None => {
                println!("{}", "Argument to this command must be a number.".red());
                return;
            }
        },

        None => {
            let mut idx = 0;
            buf.iter().for_each(|b| {
                helpers::print_line(*b, idx);
                idx += 1;
            });

            return;
        }
    };

    if let Some(i) = args.get(2) {
        let end_idx = match helpers::usize_of_str(i) {
            Some(x) => x,
            None => {
                println!("{}", "End-index must be a valid non-negative integer".red());
                return;
            }
        };

        (start_idx..=end_idx).for_each(|idx| match buf.get(idx) {
            Some(byte) => helpers::print_line(*byte, idx),
            None => {
                println!(
                    "[{:#010x}] {} {}",
                    idx.green(),
                    "Error while printing range of bytes:".red(),
                    "No such index in buffer".red()
                );

                return;
            }
        });

        return;
    }

    match buf.get(start_idx) {
        Some(b) => helpers::print_line(*b, start_idx),
        None => println!("{}", "No such index in buffer".red()),
    }
}

pub fn edit_byte(buf: &mut Vec<u8>, args: &Vec<&str>) {
    let idx = match args.get(1) {
        Some(i) => match helpers::usize_of_str(i) {
            Some(x) => x,
            None => {
                println!("{}", "Index must be a valid non-negative integer.".red());
                return;
            }
        },

        None => {
            println!(
                "{}\n{}",
                "Invalid ".red(),
                "Usage: edit <index:usize>".green()
            );

            return;
        }
    };

    let mut rl = match rustyline::DefaultEditor::new() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{} {}", "Error while creating rustyline editor:", e.red());
            return;
        }
    };

    let val = match buf.get(idx) {
        Some(v) => *v,
        None => {
            println!("{}", "No such index in buffer".red());
            return;
        }
    };

    loop {
        let input = match rl.readline_with_initial(
            &format!("{} {} {} ", "Edit".green(), idx.blue(), ">".green()),
            (&format!("{val}"), ""),
        ) {
            Ok(i) => i,
            Err(rustyline::error::ReadlineError::Eof)
            | Err(rustyline::error::ReadlineError::Interrupted) => return,
            Err(e) => {
                eprintln!(
                    "{} {}",
                    "Error while reading with rustyline editor:".red(),
                    e.red()
                );

                return;
            }
        };

        match helpers::usize_of_str(input.trim()) {
            Some(i) => match i > 0 && i <= 255 {
                true => {
                    let byte = i as u8;
                    buf[idx] = byte;
                    break;
                }

                false => {
                    println!("{}", "This number does not fit into 8 bits.".red());
                    continue;
                }
            },

            None => {
                println!("Please only enter valid non-negative integers");
                continue;
            }
        }
    }
}
