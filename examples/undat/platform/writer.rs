use libycresources::platform::Writer;

use std::fs::File;
use std::io::Write;

pub struct FileWriter<'a> {
    pub file: &'a mut File,
    limit: usize,
    buffer: Vec<u8>,
    buffered: usize,
}

pub fn from(file: &mut File, limit: usize) -> FileWriter {
    FileWriter {
        file,
        limit,
        buffer: Vec::new(),
        buffered: 0,
    }
}

impl Writer<std::io::Error> for FileWriter<'_> {
    fn append(self: &mut Self, bytes: &[u8]) -> Result<usize, std::io::Error> {
        self.buffer.extend_from_slice(bytes);
        self.buffered += bytes.len();

        if self.limit <= self.buffered {
            let _saved = self.save();

            self.buffer.clear();
            self.buffered = 0;
        }

        Ok(bytes.len())
    }
}

impl FileWriter<'_> {
    pub fn finalize(self: &mut Self) -> Result<usize, std::io::Error> {
        if 0 < self.buffered {
            return self.save();
        }

        Ok(0)
    }

    fn save(self: &mut Self) -> Result<usize, std::io::Error> {
        if let Err(error) = self.file.write(&self.buffer) {
            return Err(error);
        }

        return Ok(self.buffered);
    }
}
