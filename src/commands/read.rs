use crate::helpers;

use anyhow::bail;

pub fn read_byte(buf: &[u8], args: &[&str]) -> anyhow::Result<()> {
    // For reading ranges
    if let (Some(a1), Some(a2)) = (args.get(1), args.get(2)) {
        let start_idx = helpers::conv::usize_of_str(a1)?;
        let end_idx = helpers::conv::usize_of_str(a2)?;

        if end_idx <= start_idx {
            bail!("End-index must be larger than start-index")
        }

        if buf.len() <= end_idx {
            bail!("Trying to read out-of-bounds ({:#010x})", end_idx)
        }

        for idx in start_idx..=end_idx {
            helpers::lines::print_line(buf[idx], idx)
        }

        return Ok(());
    }

    // For reading a single byte
    if let Some(a) = args.get(1) {
        let idx = helpers::conv::usize_of_str(a)?;
        match buf.get(idx) {
            None => bail!("No such index in buffer"),
            Some(i) => return Ok(helpers::lines::print_line(*i, idx)),
        };
    };

    // For reading the whole file
    for (idx, byte) in buf.iter().enumerate() {
        helpers::lines::print_line(*byte, idx);
    }
    
    Ok(())
}
