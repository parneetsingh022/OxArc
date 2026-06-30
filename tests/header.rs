//! This file contains tests that verify the header properties
//! of archive files to ensure they are parsed and handled correctly.

mod common;

use std::{
    fs::File,
    io::{self, Read},
};
use tempdir::TempDir;

#[test]
fn header_default_fields() -> Result<(), io::Error> {
    let basedir = TempDir::new("basedir")?;

    let target_file = common::create_test_archive(&basedir)?;

    let mut file = File::open(&target_file)?;

    let mut magic = [0u8; 4];
    file.read_exact(&mut magic)?;
    assert_eq!(magic, *b"OXA!");

    let expected_version = 1 as u16;
    let mut version = [0u8; 2];
    file.read_exact(&mut version)?;
    assert_eq!(version, expected_version.to_le_bytes());

    let mut reserved = [0u8; 26];
    file.read_exact(&mut reserved)?;
    assert_eq!(reserved, [0u8; 26]);

    Ok(())
}
