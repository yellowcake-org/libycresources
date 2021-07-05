use libycresources::platform::Reader;

use std::fs::File;
use std::io::Read;
use std::io::{Seek, SeekFrom};

use std::ops::Range;

pub struct FileReader<'a> {
    pub file: &'a mut File,
    limit: usize,
    buffer: Vec<u8>,
    buffered: Range<usize>,
}

pub fn from(file: &mut File, limit: usize) -> FileReader {
    FileReader {
        file,
        limit,
        buffer: Vec::new(),
        buffered: 0..0,
    }
}

impl Reader<std::io::Error> for FileReader<'_> {
    fn read(self: &mut Self, range: Range<usize>) -> Result<Vec<u8>, std::io::Error> {
        if range.start < self.buffered.start || range.end > self.buffered.end {
            self.buffered = range.start..std::cmp::max(range.start + self.limit, range.end);
            self.buffer.clear();

            if let Err(error) = self.file.seek(SeekFrom::Start(self.buffered.start as u64)) {
                return Err(error);
            }

            let mut required = vec![0u8; range.end - range.start];
            match self.file.read_exact(&mut required) {
                Err(error) => return Err(error),
                Ok(value) => value,
            };

            self.buffer.extend_from_slice(&required);

            let extra_size = (self.buffered.end - self.buffered.start) - (range.end - range.start);
            if extra_size > 0 {
                let mut extra_buffer = vec![0u8; extra_size];
                let read = match self.file.read(&mut extra_buffer) {
                    Err(error) => return Err(error),
                    Ok(value) => value,
                };

                self.buffer.extend_from_slice(&extra_buffer);
                self.buffered.end -= extra_size - read;
            }
        }

        Ok(
            self.buffer[(range.start - self.buffered.start)..(range.end - self.buffered.start)]
                .to_vec(),
        )
    }
}
