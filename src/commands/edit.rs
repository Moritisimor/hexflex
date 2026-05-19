use crate::helpers;

use anyhow::bail;
use owo_colors::OwoColorize;

pub fn edit_byte(buf: &mut [u8], args: &[&str]) -> anyhow::Result<()> {
    let idx = match args.get(1) {
        None => bail!("Invalid amount of arguments."),
        Some(i) => match helpers::conv::usize_of_str(i) {
            Some(x) => x,
            None => bail!("{}", "Index must be a valid non-negative integer.".red()),
        },
    };

    let mut rl = rustyline::DefaultEditor::new()?;
    let val = match buf.get(idx) {
        Some(v) => *v,
        None => bail!("No such index in buffer"),
    };

    loop {
        let input = match rl.readline_with_initial(
            &format!("{} {:#010x} {} ", "Edit".green(), idx.blue(), ">".green()),
            (&format!("{:#004x}", val), ""),
        ) {
            Ok(i) => i,
            Err(rustyline::error::ReadlineError::Eof)
            | Err(rustyline::error::ReadlineError::Interrupted) => return Ok(()),
            Err(e) => bail!("Error while reading with rustyline editor: {e}"),
        };

        match helpers::conv::u8_of_str(input.trim()) {
            None => println!("{}", "Please only enter numbers from 0-255.".red()),
            Some(i) => {
                let byte = i as u8;
                buf[idx] = byte;
                break;
            }
        }
    }

    Ok(())
}

pub fn nullify(buf: &mut Vec<u8>, args: &Vec<&str>) -> anyhow::Result<()> {
    let idx = match args.get(1) {
        Some(i) => match helpers::conv::usize_of_str(i) {
            Some(x) => x,
            None => bail!("Please only enter valid non-negative integers"),
        },

        None => bail!("Invalid amount of arguments"),
    };

    if let None = buf.get(idx) {
        bail!("No such index in buffer");
    }

    buf[idx] = 0;
    println!("{} {:#010x}", "Successfully nullified".green(), idx.blue());
    Ok(())
}
