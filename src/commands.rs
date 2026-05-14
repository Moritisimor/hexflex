use crate::helpers;

use owo_colors::OwoColorize;

pub fn read_byte(buf: &Vec<u8>, args: &Vec<&str>) {
    let idx = match args.get(1) {
        Some(i) => {
            match i.parse::<i64>() {
                Ok(n) => n,
                Err(_) => {
                    println!("{}", "Argument to this command must be a number.".red());
                    return
                }
            }
        }
        
        None => {
            let mut idx = 0;
            for byte in buf {
                helpers::print_line(*byte, idx);
                idx += 1;
            }
            
            return
        }
    };
    
    match buf.get(idx as usize) {
        Some(b) => helpers::print_line(*b, idx),
        None => println!("{}", "No such index in buffer".red())
    }
}
