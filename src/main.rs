mod commands;
mod flags;
mod helpers;

use std::fs;

use anyhow::bail;
use clap::Parser;
use owo_colors::OwoColorize;
use rustyline::error::ReadlineError;

use crate::flags::Flags;

fn main() -> anyhow::Result<()> {
    let flags = Flags::parse();

    let data = fs::read(&flags.input_file)?;
    if helpers::is_elf(&data) {
        println!("{}", "This file is probably ELF!".blue())
    }

    if let Some(file_name) = flags.output_file {
        let mut file_buf = String::new();
        let mut line_buf = String::new();
        let mut idx = 0;

        data.iter().for_each(|b| {
            helpers::format_line(*b, idx, &mut line_buf);
            file_buf += &line_buf;
            idx += 1;
        });

        std::fs::write(&file_name, file_buf)?;
        println!(
            "{} {}",
            "Successfully saved content to:".green(),
            &file_name.magenta()
        );

        return Ok(());
    }

    let mut rl = rustyline::DefaultEditor::new()?;
    let prompt = format!(
        "{}{}{} {} {} ",
        "[".blue(),
        &flags.input_file.green(),
        "]".blue(),
        "HexFlex".blue(),
        ">>".green()
    );

    loop {
        let input = match rl.readline(&prompt) {
            Err(ReadlineError::Interrupted) => {
                println!(
                    "{} {}",
                    "Interrupted.".red(),
                    "Enter 'exit' to exit.".green()
                );

                continue;
            }

            Err(ReadlineError::Eof) => break,
            Err(e) => bail!("{e}"),

            Ok(i) => i,
        };

        let fields: Vec<&str> = input.split_whitespace().collect();
        if fields.is_empty() {
            continue;
        }

        match fields[0] {
            "exit" | "q" => break,
            "read" | "r" => commands::read_byte(&data, &fields),
            _ => println!("{} {}", "Unknown command:".red(), fields[0].blue()),
        }
    }

    println!("{}", "Bye".green());
    Ok(())
}
