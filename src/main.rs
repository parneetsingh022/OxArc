pub mod arcwriter;
pub mod format;

use std::{
    env,
    fs,
    io,
    path::{Path, PathBuf},
};

use crate::arcwriter::ArchiveWriter;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <folder> <archive>", args[0]);
        std::process::exit(1);
    }

    let folder = PathBuf::from(&args[1]);
    let archive = &args[2];

    if !folder.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("not a folder: {}", folder.display()),
        ));
    }

    let mut archive_writer = ArchiveWriter::new(archive)?;

    add_folder(&mut archive_writer, &folder)?;

    archive_writer.finish()?;

    Ok(())
}

fn add_folder(archive_writer: &mut ArchiveWriter, folder: &Path) -> Result<(), io::Error> {
    for entry in fs::read_dir(folder)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            add_folder(archive_writer, &path)?;
        } else if path.is_file() {
            println!("a {}", path.display());
            archive_writer.add_file(&path)?;
        }
    }

    Ok(())
}