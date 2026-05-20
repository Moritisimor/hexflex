use anyhow::bail;

use crate::helpers;

pub fn delete(buf: &mut Vec<u8>, args: &[&str]) -> anyhow::Result<()> {
    if let (Some(a1), Some(a2)) = (args.get(1), args.get(2)) {
        let start_idx = helpers::conv::usize_of_str(a1)?;
        let end_idx = helpers::conv::usize_of_str(a2)?;

        let (Some(_), Some(_)) = (buf.get(start_idx), buf.get(end_idx)) else {
            bail!("Cannot delete this byte sequence as it is out of bounds")
        };

        if end_idx <= start_idx {
            bail!("End-index must be larger than start-index")
        }

        for _ in start_idx..=end_idx {
            buf.remove(start_idx);
        }

        return Ok(());
    }

    if let Some(a) = args.get(1) {
        let idx = helpers::conv::usize_of_str(a)?;
        if let None = buf.get(idx) {
            bail!("Cannot delete byte at this sequence as it is out of bounds")
        };

        buf.remove(idx);
        return Ok(());
    }

    bail!("Expected at least one argument, got none")
}
