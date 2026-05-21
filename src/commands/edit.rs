use crate::helpers;

use anyhow::bail;
use owo_colors::OwoColorize;

pub fn edit_byte(buf: &mut [u8], args: &[&str]) -> anyhow::Result<()> {
    let idx = match args.get(1) {
        None => bail!("Invalid amount of arguments."),
        Some(i) => helpers::conv::usize_of_str(i)?,
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

        let byte = helpers::conv::u8_of_str(input.trim())?;
        buf[idx] = byte;
        break;
    }

    Ok(())
}

pub fn nullify(buf: &mut Vec<u8>, args: &Vec<&str>) -> anyhow::Result<()> {
    if let (Some(a1), Some(a2)) = (args.get(1), args.get(2)) {
        let start_idx = helpers::conv::usize_of_str(a1)?;
        let end_idx = helpers::conv::usize_of_str(a2)?;

        let (Some(_), Some(_)) = (buf.get(start_idx), buf.get(end_idx)) else {
            bail!("Cannot delete this byte sequence as it is out of bounds")
        };

        if end_idx <= start_idx {
            bail!("End-index must be larger than start-index")
        }

        for i in start_idx..=end_idx {
            buf[i] = 0
        }

        println!(
            "{} {:#010x} {} {:#010x}",
            "Successfully nullified".green(),
            start_idx.blue(),
            "-".green(),
            end_idx.blue()
        );

        return Ok(())
    }

    let idx = match args.get(1) {
        Some(i) => helpers::conv::usize_of_str(i)?,
        None => bail!("Expected at least one argument"),
    };

    if let None = buf.get(idx) {
        bail!("No such index in buffer");
    }

    buf[idx] = 0;
    println!("{} {:#010x}", "Successfully nullified".green(), idx.blue());
    Ok(())
}
