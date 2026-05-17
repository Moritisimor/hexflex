use std::io;

fn make_error(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, msg)
}

pub fn get_hist_file_path() -> anyhow::Result<String> {
    let mut home_dir = std::env::home_dir()
        .ok_or(make_error("Error while getting home directory"))?
        .to_str()
        .ok_or(make_error("Error while converting home directory to str"))?
        .to_string();

    home_dir.push_str(std::path::MAIN_SEPARATOR_STR);
    home_dir.push_str(".hexflexhist");
    Ok(home_dir)
}
