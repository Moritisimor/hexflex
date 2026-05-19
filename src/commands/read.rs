use crate::helpers;

use anyhow::bail;

pub fn read_byte(buf: &[u8], args: &[&str]) -> anyhow::Result<()> {
    let start_idx = match args.get(1) {
        Some(i) => match helpers::conv::usize_of_str(i) {
            Some(n) => n,
            None => bail!("Arguments to this command must be a numbers"),
        },

        None => {
            let mut idx = 0;
            buf.iter().for_each(|b| {
                helpers::lines::print_line(*b, idx);
                idx += 1;
            });

            return Ok(());
        }
    };

    if let Some(i) = args.get(2) {
        let end_idx = match helpers::conv::usize_of_str(i) {
            Some(x) => x,
            None => bail!("End-index must be a valid non-negative integer"),
        };

        if end_idx <= start_idx {
            bail!("End-index must be larger than start index")
        }

        for idx in start_idx..=end_idx {
            match buf.get(idx) {
                Some(byte) => helpers::lines::print_line(*byte, idx),
                None => bail!(
                    "[{:#010x}] {} {}",
                    idx,
                    "Error while printing range of bytes:",
                    "No such index in buffer"
                ),
            }
        }

        return Ok(());
    }

    match buf.get(start_idx) {
        Some(b) => Ok(helpers::lines::print_line(*b, start_idx)),
        None => bail!("No such index in buffer"),
    }
}
