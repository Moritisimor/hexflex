use crate::helpers;

use owo_colors::OwoColorize;

pub fn output(data: &[u8], file_name: &str) -> anyhow::Result<()> {
    let mut file_buf = String::new();
    helpers::lines::format_bytes(data, &mut file_buf);
    std::fs::write(&file_name, file_buf)?;

    println!(
        "{} {}",
        "Successfully saved content to:".green(),
        &file_name.magenta()
    );

    Ok(())
}
