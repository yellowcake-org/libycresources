use super::*;

mod prototype;

pub fn list<S: Read + Seek, P: PrototypeProvider>(source: &mut S, provider: &P, elevations: &[Option<()>]) ->
Result<HashSet<blueprint::prototype::Instance>, errors::Error> {
    let mut list = HashSet::new();
    let count = source.read_u32::<BigEndian>()?;

    for _ in elevations {
        for _ in 0..source.read_u32::<BigEndian>()? {
            if !list.insert(prototype::instance(source, provider)?) { return Err(errors::Error::Format); }
        }
    }

    if list.len() != count as usize {
        return Err(errors::Error::Format);
    }

    Ok(list)
}