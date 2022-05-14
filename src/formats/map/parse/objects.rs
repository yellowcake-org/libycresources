use super::*;

mod object;

pub fn list<S: Read + Seek>(source: &mut S, elevations: &[Option<()>]) -> Result<HashSet<state::object::Instance>, errors::Error> {
    let mut list = HashSet::new();

    let mut total_count_bytes = [0u8; 4];
    match source.read_exact(&mut total_count_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let total_count = u32::from_be_bytes(total_count_bytes);

    for e in elevations {
        if e.is_some() {
            let mut count_bytes = [0u8; 4];
            match source.read_exact(&mut count_bytes) {
                Err(error) => return Err(errors::Error::Read(error)),
                Ok(value) => value,
            };

            for _ in 0..u32::from_be_bytes(count_bytes) {
                match object::instance(source) {
                    Ok(value) => {
                        if !list.insert(value) { return Err(errors::Error::Format); }
                    }
                    Err(error) => return Err(error)
                };
            }
        }
    }

    if list.len() != total_count as usize {
        return Err(errors::Error::Format);
    }

    Ok(list)
}