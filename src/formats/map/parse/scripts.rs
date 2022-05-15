use super::*;

mod script;

pub fn list<S: Read + Seek>(source: &mut S) -> Result<HashSet<blueprint::script::Instance>, errors::Error> {
    let mut list = HashSet::new();

    for type_raw in 0..5 {
        let mut read = 0;
        let count = source.read_u32::<BigEndian>()? as usize;

        const BATCH_LENGTH: usize = 16;
        let batches = count / BATCH_LENGTH + (count % BATCH_LENGTH != 0) as usize;

        for page in 0..batches {
            for record in 0..BATCH_LENGTH {
                if (page * BATCH_LENGTH) + record < count {
                    list.insert(script::instance(source, type_raw)?);
                } else {
                    script::skip(source)?;
                }
            }

            read += source.read_u32::<BigEndian>()? as usize;
            source.seek(SeekFrom::Current(4))?;
        }

        if read != count {
            return Err(errors::Error::Format);
        }
    }

    Ok(list)
}