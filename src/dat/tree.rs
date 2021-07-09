use super::super::platform::Reader;
use super::{Directory, File};

use std::convert::TryInto;
use std::mem::size_of;

#[derive(Debug)]
pub enum Error<R> {
    Read(R),
    Format,
    Reader,
}

pub fn entries<R, E>(reader: &mut R) -> Result<Directory, Error<E>>
where
    R: Reader<E>,
{
    let mut offset = 0;

    let count = u32::from_be_bytes(
        match reader
            .read(offset..offset + size_of::<u32>())
            .map(|vec| vec.try_into())
        {
            Err(error) => return Err(Error::Read(error)),
            Ok(value) => match value {
                Err(_) => return Err(Error::Reader),
                Ok(value) => value,
            },
        },
    );

    offset += size_of::<u32>();
    offset += 3 * size_of::<u32>(); // skip attributes

    let mut tree: Directory = Directory {
        name: String::from("."),
        files: Vec::new(),
        children: Vec::new(),
    };
    // let mut flatten = Vec::<&mut Directory>::with_capacity(count as usize);

    for _ in 0..count as usize {
        let length = u8::from_be_bytes(
            match reader
                .read(offset..offset + size_of::<u8>())
                .map(|vec| vec.try_into())
            {
                Err(error) => return Err(Error::Read(error)),
                Ok(value) => match value {
                    Err(_) => return Err(Error::Reader),
                    Ok(value) => value,
                },
            },
        ) as usize;

        offset += size_of::<u8>();

        let mut path = match String::from_utf8(match reader.read(offset..offset + length) {
            Err(_) => return Err(Error::Reader),
            Ok(value) => value,
        }) {
            Err(_) => return Err(Error::Format),
            Ok(value) => value,
        };

        offset += length;

        // i have no idea why original Fallout™ archives use . for root folder and
        // do not use .\ at start for all it's children, so...
        if path != "." {
            path = String::from(".\\") + &path;
        }

        let mut current: &mut Directory = &mut tree;
        for (index, component) in path.split('\\').enumerate() {
            if index > 0 {
                if let Some(existed) = current.children.iter_mut().find(|n| n.name == component) {
                    current = existed;
                } else {
                    // current.children.push(Directory {
                    //     name: String::from(component),
                    //     files: Vec::new(),
                    //     children: Vec::new(),
                    // });

                    // current = current.children.last_mut().unwrap();
                }
            }
        }

        // if let Some(mut node) = current {
        //     flatten[index] = &mut node;
        // } else {
        //     return Err(Error::Format);
        // }
    }

    // assert_ne!(flatten.len(), count);

    // for dir in &flatten {
    //     let file_count = u32::from_be_bytes(
    //         match reader
    //             .read(offset..offset + size_of::<u32>())
    //             .map(|vec| vec.try_into())
    //         {
    //             Err(error) => return Err(Error::Read(error)),
    //             Ok(value) => match value {
    //                 Err(_) => return Err(Error::Reader),
    //                 Ok(value) => value,
    //             },
    //         },
    //     );

    //     offset += size_of::<u32>();
    //     offset += 3 * size_of::<u32>(); // skip attributes

    //     for _ in 0..file_count {
    //         let length = u8::from_be_bytes(
    //             match reader
    //                 .read(offset..offset + size_of::<u8>())
    //                 .map(|vec| vec.try_into())
    //             {
    //                 Err(error) => return Err(Error::Read(error)),
    //                 Ok(value) => match value {
    //                     Err(_) => return Err(Error::Reader),
    //                     Ok(value) => value,
    //                 },
    //             },
    //         ) as usize;

    //         offset += size_of::<u8>();

    //         let name: String = match String::from_utf8(match reader.read(offset..offset + length) {
    //             Err(_) => return Err(Error::Reader),
    //             Ok(value) => value,
    //         }) {
    //             Err(_) => return Err(Error::Format),
    //             Ok(value) => value,
    //         };

    //         offset += length;
    //         offset += size_of::<u32>(); // skip attributes

    //         let start = u32::from_be_bytes(
    //             match reader
    //                 .read(offset..offset + size_of::<u32>())
    //                 .map(|vec| vec.try_into())
    //             {
    //                 Err(error) => return Err(Error::Read(error)),
    //                 Ok(value) => match value {
    //                     Err(_) => return Err(Error::Reader),
    //                     Ok(value) => value,
    //                 },
    //             },
    //         ) as usize;
    //         offset += size_of::<u32>();

    //         let size = u32::from_be_bytes(
    //             match reader
    //                 .read(offset..offset + size_of::<u32>())
    //                 .map(|vec| vec.try_into())
    //             {
    //                 Err(error) => return Err(Error::Read(error)),
    //                 Ok(value) => match value {
    //                     Err(_) => return Err(Error::Reader),
    //                     Ok(value) => value,
    //                 },
    //             },
    //         ) as usize;
    //         offset += size_of::<u32>();

    //         let packed_size = u32::from_be_bytes(
    //             match reader
    //                 .read(offset..offset + size_of::<u32>())
    //                 .map(|vec| vec.try_into())
    //             {
    //                 Err(error) => return Err(Error::Read(error)),
    //                 Ok(value) => match value {
    //                     Err(_) => return Err(Error::Reader),
    //                     Ok(value) => value,
    //                 },
    //             },
    //         ) as usize;
    //         offset += size_of::<u32>();

    //         dir.files.push(File {
    //             name: name,
    //             range: start..start + {
    //                 if packed_size > 0 {
    //                     packed_size
    //                 } else {
    //                     size
    //                 }
    //             },
    //             size: size as usize,
    //         })
    //     }
    // }

    Ok(tree)
}
