use std::fs;

use crate::helpers;

pub fn reverse(input_file: &str, output_file: &str) -> anyhow::Result<()> {
    let mut acc: Vec<u8> = vec![];
    let content = fs::read_to_string(input_file)?;

    
    for word in content.split_whitespace() {
        acc.push(helpers::conv::u8_of_str(word)?);
    }

    fs::write(output_file, acc)?;
    Ok(())
}
