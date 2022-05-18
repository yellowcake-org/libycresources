use super::*;

mod prototype;

pub fn list<S: Read + Seek, P: PrototypeProvider>
(source: &mut S, provider: &P, elevations: &[Option<()>], read_ladders_map: bool) ->
Result<Vec<blueprint::prototype::Instance>, errors::Error> {
    let mut list = Vec::new();
    let count = source.read_u32::<BigEndian>()?;

    for _ in elevations {
        for _ in 0..source.read_u32::<BigEndian>()? {
            list.push(prototype::instance(source, provider, read_ladders_map)?);
        }
    }

    if list.len() != count as usize {
        return Err(errors::Error::Format);
    }

    Ok(list)
}