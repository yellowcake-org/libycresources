use std::io::Read;

use super::*;

pub fn tuple<S: Read>(source: &mut S) -> Result<(HashSet<common::Flag>, [Option<()>; 3]), errors::Error> {
    let mut flags_bytes = [0u8; 4];
    source.read_exact(&mut flags_bytes)?;

    let mut flags: HashSet<common::Flag> = HashSet::new();
    let mut elevations = [None, None, None];

    if (flags_bytes[3] & 0x01) != 0x00 {
        if !flags.insert(common::Flag::Save) {
            return Err(errors::Error::Format);
        }
    }

    if (flags_bytes[3] & 0x02) == 0x00 {
        elevations[0] = Some(());
    }

    if (flags_bytes[3] & 0x04) == 0x00 {
        elevations[1] = Some(());
    }

    if (flags_bytes[3] & 0x08) == 0x00 {
        elevations[2] = Some(());
    }

    Ok((flags, elevations))
}