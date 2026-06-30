use assert_cmd::Command;
use std::{fs, io, path::PathBuf};
use tempdir::TempDir;

pub fn create_test_archive(basedir: &TempDir) -> Result<PathBuf, io::Error> {
    let source_dir = basedir.path().join("dir");
    fs::create_dir(&source_dir)?;

    fs::write(source_dir.join("file1.txt"), "Content")?;
    fs::write(source_dir.join("file2.txt"), "Contents")?;

    let target_file = basedir.path().join("dir.oxa");

    Command::cargo_bin("oxarc")
        .unwrap()
        .arg(&source_dir)
        .arg(&target_file)
        .assert()
        .success();

    Ok(target_file)
}
