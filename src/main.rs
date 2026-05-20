pub mod commands;
pub mod flags;
pub mod helpers;
pub mod modes;

use std::fs;

use anyhow::bail;
use clap::Parser;
use owo_colors::OwoColorize;

use crate::flags::Flags;

fn main() -> anyhow::Result<()> {
    let flags = Flags::parse();
    let data = fs::read(&flags.input_file)?;

    if let Some(file_name) = flags.output_file {
        match flags.reverse {
            false => modes::output::output(&data, &file_name)?,
            true => modes::reverse::reverse(&flags.input_file, &file_name)?,
        };

        return Ok(());
    }

    if flags.reverse {
        bail!("Setting the --reverse flag requires the --output-file flag to be set as well")
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
