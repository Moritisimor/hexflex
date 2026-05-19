use owo_colors::OwoColorize;

pub fn save(buf: &[u8], args: &[&str], fallback: &str) {
    let path = match args.get(1) {
        Some(i) => *i,
        None => fallback,
    };

    match std::fs::write(path, buf) {
        Ok(_) => println!(
            "{} '{}'",
            "Successfully saved buffer to".green(),
            path.blue()
        ),

        Err(e) => {
            eprintln!("{} {}", "Error while saving buffer to file:".red(), e.red())
        }
    }
}
