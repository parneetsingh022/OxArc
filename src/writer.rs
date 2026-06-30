use crate::format::{ArchiveFooter, ArchiveHeader, FileEntry};
use std::fs::File;
use std::io::{self, BufWriter, Seek, Write};
use std::path::Path;

pub struct ArchiveWriter {
    writer: BufWriter<File>,
    index: Vec<FileEntry>,
    index_size: u64,
}

impl ArchiveWriter {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, io::Error> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        let header = ArchiveHeader::new();

        writer.write(&header.as_bytes())?;

        Ok(Self {
            writer: writer,
            index: Vec::new(),
            index_size: 0,
        })
    }

    pub fn add_file(&mut self, path: impl AsRef<Path>) -> Result<(), io::Error> {
        let mut file = File::open(&path)?;

        let start_offset = self.writer.stream_position()?;

        io::copy(&mut file, &mut self.writer)?;

        let end_offset = self.writer.stream_position()?;
        let uncompressed_size = (end_offset - start_offset) as u64;

        let file_entry = FileEntry::new(path, uncompressed_size, start_offset);
        self.index.push(file_entry);

        Ok(())
    }

    fn write_file_index(&mut self) -> Result<(), io::Error> {
        for item in &self.index {
            let index = item.as_bytes();
            self.index_size += index.len() as u64;

            self.writer.write_all(&index)?;
        }

        Ok(())
    }

    pub fn finish(&mut self) -> Result<(), io::Error> {
        let index_offset = self.writer.stream_position()?;

        self.write_file_index()?;

        let footer = ArchiveFooter::new(index_offset, self.index_size);

        self.writer.write_all(&footer.as_bytes())?;
        self.writer.flush()?;

        Ok(())
    }
}
