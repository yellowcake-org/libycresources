use super::*;

mod script;

pub fn list<S: Read + Seek>(source: &mut S) -> Result<Vec<blueprint::script::Instance>, errors::Error> {
    let mut list = Vec::new();

    for type_raw in 0..5 {
        let mut read = 0;
        let count = source.read_u32::<BigEndian>()? as usize;

        const BATCH_LENGTH: usize = 16;
        let batches = count / BATCH_LENGTH + (count % BATCH_LENGTH != 0) as usize;

        for page in 0..batches {
            for record in 0..BATCH_LENGTH {
                if (page * BATCH_LENGTH) + record < count {
                    list.push(script::instance(source, type_raw)?);
                } else {
                    script::skip(source)?;
                }
            }

            read += usize::try_from(source.read_u32::<BigEndian>()?).map_err(|_| errors::Error::Format)?;
            source.seek(SeekFrom::Current(4))?;
        }

        if read != count {
            return Err(errors::Error::Format);
        }
    }

    Ok(list)
}