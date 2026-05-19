pub mod commands;
pub mod flags;
pub mod helpers;
pub mod modes;

use std::fs;

use clap::Parser;
use owo_colors::OwoColorize;

use crate::{flags::Flags, helpers::hist::get_hist_file_path};

fn main() -> anyhow::Result<()> {
    let flags = Flags::parse();
    let data = fs::read(&flags.input_file)?;

    println!(
        "{} {} {}",
        "Read".blue(),
        data.len().green(),
        "Bytes".blue()
    );

    if helpers::info::is_elf(&data) {
        println!("{}", "This file is probably ELF!".blue())
    }

    if let Some(file_name) = flags.output_file {
        modes::output::output(&data, &file_name)?;
        return Ok(());
    }

    let hist_file_path = get_hist_file_path()?;
    let mut rl = rustyline::DefaultEditor::new()?;
    if rl.load_history(&hist_file_path).is_err() {
        println!("{}", "No histfile yet.".yellow());
    }

    let prompt = format!(
        "{}{}{} {} {} ",
        "[".blue(),
        &flags.input_file.green(),
        "]".blue(),
        "HexFlex".blue(),
        ">>".green()
    );

    modes::repl::repl(&prompt, data, &flags.input_file)
}
