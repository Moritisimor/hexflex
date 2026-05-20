use owo_colors::OwoColorize;
use rustyline::error::ReadlineError;

use crate::{commands, helpers};
use anyhow::bail;

pub fn repl(prompt: &str, mut data: Vec<u8>, file_name: &str) -> anyhow::Result<()> {
    let hist_file_path = helpers::hist::get_hist_file_path()?;
    let mut rl = rustyline::DefaultEditor::new()?;
    if rl.load_history(&hist_file_path).is_err() {
        println!("{}", "No histfile yet.".yellow());
    }

    println!(
        "{} {} {}",
        "Read".blue(),
        data.len().green(),
        "Bytes".blue()
    );

    if helpers::info::is_elf(&data) {
        println!("{}", "This file is probably ELF!".blue())
    }

    loop {
        let input = match rl.readline(prompt) {
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

        rl.add_history_entry(&input)?;
        let fields: Vec<&str> = input.split_whitespace().collect();
        if fields.is_empty() {
            continue;
        }

        let res = match fields[0] {
            "quit" | "exit" | "q" => break,
            "read" | "r" => commands::read::read_byte(&data, &fields),
            "edit" | "e" => commands::edit::edit_byte(&mut data, &fields),
            "save" | "s" => commands::save::save(&data, &fields, file_name),
            "delete" | "d" => commands::delete::delete(&mut data, &fields),
            "nullify" | "n" => commands::edit::nullify(&mut data, &fields),
            "findbytes" | "fb" => commands::find::find_bytes(&data, &fields),
            "findstring" | "fs" => commands::find::find_string(&data, &input),
            "clear" | "c" => {
                println!("\x1b[H\x1b[2J\x1b[3J");
                continue;
            }

            _ => {
                println!("{} {}", "Unknown command:".red(), fields[0].blue());
                continue;
            }
        };

        if let Err(e) = res {
            println!("{} {}", "Error while executing:".red(), e.yellow())
        }
    }

    rl.save_history(&hist_file_path)?;
    println!("{}", "Bye".green());
    Ok(())
}
