use std::io::{Read, Seek};
use std::path::PathBuf;

use ycresources::common::types::errors::Error;
use ycresources::formats::dat;

pub(crate) fn tree<R>(reader: &mut R, tree: &dat::Directory, output: &String) -> Result<(), Error> where R: Read + Seek {
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

            let directory = path.parent().ok_or(Error::Format)?;

            std::fs::create_dir_all(&directory)?;
            let mut created = std::fs::File::create(&path)?;

            let mut writer = std::io::BufWriter::with_capacity(1 * 1024 * 1024, &mut created);
            dat::extract::file(reader, &file, &mut writer)?;
        }
    }

    Ok(())
}
