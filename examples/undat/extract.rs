use super::platform;
use libycresources::dat;

use std::path::PathBuf;

#[derive(Debug)]
pub(crate) enum Error<E> {
    Path,
    Buffer,
    Decompress,
    Read(E),
    Write(std::io::Error),
}

pub(crate) fn tree<R, E>(
    reader: &mut R,
    tree: &dat::Directory,
    output: &String,
) -> Result<(), Error<E>>
where
    R: libycresources::platform::Reader<E>,
{
    let mut path = PathBuf::new();

    for (depth, _, directory) in tree.iter() {
        if depth > path.components().count() {
            path.push(String::from(&directory.name));
        } else {
            for _ in 0..path.components().count() - depth {
                path.pop();
            }

            if !path.ends_with(String::from(&directory.name)) {
                path.push(String::from(&directory.name));
            }
        }

        for file in &directory.files {
            let root = std::path::Path::new(&output);
            let full = root.join(&path).join(&file.name);
            let path = full.as_path();

            println!("{:}", path.display());

            let directory = match path.parent() {
                None => return Err(Error::Path),
                Some(directory) => directory,
            };

            if let Err(error) = std::fs::create_dir_all(&directory) {
                return Err(Error::Write(error));
            }

            let mut created = match std::fs::File::create(&path) {
                Err(error) => return Err(Error::Write(error)),
                Ok(created) => created,
            };

            let buffer_write_size: usize = 1 * 1024 * 1024;
            let mut writer = platform::writer::from(&mut created, buffer_write_size);

            if let Err(error) = dat::extract::file(reader, &file, &mut writer) {
                return match error {
                    dat::extract::Error::Reader => Err(Error::Buffer),
                    dat::extract::Error::Decompress => Err(Error::Decompress),
                    dat::extract::Error::Read(error) => Err(Error::Read(error)),
                    dat::extract::Error::Write(error) => Err(Error::Write(error)),
                };
            }

            if let Err(error) = writer.finalize() {
                return Err(Error::Write(error));
            }
        }
    }

    Ok(())
}
