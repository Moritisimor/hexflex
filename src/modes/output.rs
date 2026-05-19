use crate::helpers;

use owo_colors::OwoColorize;

pub fn output(data: &[u8], file_name: &str) -> anyhow::Result<()> {
    let mut file_buf = String::new();
    let mut line_buf = String::new();
    let mut idx = 0;

    data.iter().for_each(|b| {
        helpers::lines::format_line(*b, idx, &mut line_buf);
        file_buf += &line_buf;
        idx += 1;
    });

    std::fs::write(&file_name, file_buf)?;
    println!(
        "{} {}",
        "Successfully saved content to:".green(),
        &file_name.magenta()
    );

    Ok(())
}
