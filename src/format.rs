use std::path::Path;

pub const OXARC_MAGIC: [u8; 4] = *b"OXAR";
pub const OXARC_VERSION: u32 = 1;

pub struct FileFooter {
    // 4-byte magic number to identify OxArc-file
    pub magic: [u8; 4],
    pub version: u32,
    pub index_offset: u64,
}

impl FileFooter {
    pub const SIZE: usize = 4 + 4 + 8;

    pub fn as_zero_bytes(&self) -> [u8; Self::SIZE] {
        let bytes = [0u8; Self::SIZE];

        bytes
    }

    pub fn as_bytes(&self) -> [u8; Self::SIZE] {
        let mut bytes = [0u8; Self::SIZE];
        bytes[0..4].copy_from_slice(&self.magic);
        bytes[4..8].copy_from_slice(&self.version.to_le_bytes()); // Explicit Little-Endian
        bytes[8..16].copy_from_slice(&self.index_offset.to_le_bytes()); // Explicit Little-Endian
        bytes
    }
}

pub struct FileEntry {
    pub path_size: u16,
    pub file_name: String,
    pub uncompressed_size: u64,
    pub offset: u64,
}

impl FileEntry {
    pub const PATH_SIZE_BYTES: usize = 2;
    pub const SIZE_BYTES: usize = 8;
    pub const OFFSET_BYTES: usize = 8;

    pub fn serialized_size(&self) -> usize {
        Self::PATH_SIZE_BYTES
            + self.file_name.as_bytes().len()
            + Self::SIZE_BYTES
            + Self::OFFSET_BYTES
    }

    pub fn new(
        path: impl AsRef<Path>,
        uncompressed_size: u64,
        offset: u64,
    ) -> Self {
        let path_str = path.as_ref().to_string_lossy().to_string();
        let path_size = path_str.as_bytes().len();

        assert!(
            path_size <= u16::MAX as usize,
            "path is too long to fit in u16 path_size"
        );

        Self {
            path_size: path_size as u16,
            file_name: path_str,
            uncompressed_size,
            offset,
        }
    }
    
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.serialized_size());

        bytes.extend_from_slice(&self.path_size.to_le_bytes());
        bytes.extend_from_slice(self.file_name.as_bytes());
        bytes.extend_from_slice(&self.uncompressed_size.to_le_bytes());
        bytes.extend_from_slice(&self.offset.to_le_bytes());

        bytes
    }
} 
