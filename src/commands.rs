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
            for byte in buf {
                helpers::print_line(*byte, idx);
                idx += 1;
            }

            return;
        }
    };

    match args.get(2) {
        None => match buf.get(start_idx) {
            Some(b) => helpers::print_line(*b, start_idx),
            None => println!("{}", "No such index in buffer".red()),
        },

        Some(i) => match helpers::usize_of_str(i) {
            Some(end_idx) => {
                for idx in start_idx..=end_idx {
                    match buf.get(idx) {
                        Some(byte) => helpers::print_line(*byte, idx),
                        None => {
                            println!(
                                "{} {}",
                                "Error while printing range of bytes:".red(),
                                "No such index in buffer"
                            );

                            return;
                        }
                    }
                }
            }

            None => {
                println!("{}", "End-index must be a valid non-negative integer".red());
                return;
            }
        },
    };
}
