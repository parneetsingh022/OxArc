use crate::format::{FileFooter, FileEntry, OXARC_MAGIC, OXARC_VERSION};
use std::fs::File;
use std::io::{self, BufWriter, Seek, Write};
use std::path::Path;

pub struct ArchiveWriter {
    writer: BufWriter<File>,
    index : Vec<FileEntry>
}

impl ArchiveWriter {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, io::Error> {
        let file = File::create(path)?;

        Ok(Self {
            writer: BufWriter::new(file),
            index : Vec::new()
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

    pub fn write_file_index(&mut self) -> Result<(), io::Error> {
        for item in &self.index {
            self.writer.write_all(&item.as_bytes())?;
        }
        

        Ok(())
    }

    pub fn finish(&mut self) -> Result<(), io::Error> {
        let index_offset = self.writer.stream_position()?;

        self.write_file_index()?;

        let footer = FileFooter {
            magic: OXARC_MAGIC,
            version: OXARC_VERSION,
            index_offset,
        };

        self.writer.write_all(&footer.as_bytes())?;
        self.writer.flush()?;

        Ok(())
    }
}
