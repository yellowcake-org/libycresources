use super::platform;
use libycresources::dat;

#[derive(Debug)]
pub(crate) enum Error<E> {
    Path,
    Buffer,
    Decompress,
    Read(E),
    Write(std::io::Error),
}

pub(crate) fn entries<R, E>(
    reader: &mut R,
    entries: &[dat::Entry],
    output: &String,
) -> Result<(), Error<E>>
where
    R: libycresources::platform::Reader<E>,
{
    for item in entries {
        println!("Extracting {:?}...", item.path);

        let root = std::path::Path::new(&output);
        let joined = root.join(&item.path);
        let path = joined.as_path();

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

        if let Err(error) = dat::extract::entry(reader, &item, &mut writer) {
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

    Ok(())
}
