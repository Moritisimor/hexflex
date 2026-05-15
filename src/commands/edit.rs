use crate::helpers;

use owo_colors::OwoColorize;

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
                "Invalid amount of arguments.".red(),
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
            &format!("{} {:#010x} {} ", "Edit".green(), idx.blue(), ">".green()),
            (&format!("{:#004x}", val), ""),
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
            None => println!("{}", "Please only enter valid non-negative integers".red()),
            Some(i) => match i <= 255 {
                false => println!("{}", "This number does not fit into a byte.".red()),
                true => {
                    let byte = i as u8;
                    buf[idx] = byte;
                    break;
                }
            },
        }
    }
}
