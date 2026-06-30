//! This file contains tests that verify the footer properties
//! of archive files to ensure they are parsed and handled correctly.

mod common;

use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom},
};
use tempdir::TempDir;

#[test]
fn test_magic_in_footer() -> Result<(), io::Error> {
    let basedir = TempDir::new("basedir")?;
    let target_file = common::create_test_archive(&basedir)?;

    let mut file = File::open(&target_file)?;
    file.seek(SeekFrom::End(-32))?;

    let mut magic = [0u8; 4];

    file.read_exact(&mut magic)?;

    assert_eq!(magic, *b"OXAE");

    Ok(())
}
