use owo_colors::OwoColorize;

pub fn save(buf: &[u8], args: &[&str], fallback: &str) -> anyhow::Result<()> {
    let path = match args.get(1) {
        Some(i) => *i,
        None => fallback,
    };

    std::fs::write(path, buf)?;
    println!(
        "{} '{}'",
        "Successfully saved buffer to".green(),
        path.blue()
    );

    Ok(())
}
