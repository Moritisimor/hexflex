use owo_colors::OwoColorize;

pub fn save(buf: &Vec<u8>, path: &str) {
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
