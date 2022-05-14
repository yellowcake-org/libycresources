use super::*;

mod script;

pub fn list<S: Read + Seek>(source: &mut S) -> Result<HashSet<blueprint::script::Instance>, errors::Error> {
    let mut list = HashSet::new();

    for type_raw in 0..5 {
        let mut count_bytes = [0u8; 4];
        match source.read_exact(&mut count_bytes) {
            Err(error) => return Err(errors::Error::Read(error)),
            Ok(value) => value,
        };

        let mut read = 0;
        let count = u32::from_be_bytes(count_bytes) as usize;

        const BATCH_LENGTH: usize = 16;
        let batches = count / BATCH_LENGTH + (count % BATCH_LENGTH != 0) as usize;

        for page in 0..batches {
            for record in 0..BATCH_LENGTH {
                if (page * BATCH_LENGTH) + record < count {
                    match script::instance(source, type_raw) {
                        Ok(value) => { list.insert(value); }
                        Err(_) => return Err(errors::Error::Format)
                    }
                } else {
                    if let Err(_) = script::skip(source) {
                        return Err(errors::Error::Format);
                    }
                }
            }

            let mut check_count_bytes = [0u8; 4];
            match source.read_exact(&mut check_count_bytes) {
                Err(error) => return Err(errors::Error::Read(error)),
                Ok(value) => value,
            };

            read += u32::from_be_bytes(check_count_bytes) as usize;

            if let Err(error) = source.seek(SeekFrom::Current(4)) {
                return Err(errors::Error::Read(error));
            }
        }

        if read != count {
            return Err(errors::Error::Format);
        }
    }

    Ok(list)
}