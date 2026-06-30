use anyhow::{Context, Result};
use walkdir::WalkDir;

use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use clap::Parser;

use crate::writer::ArchiveWriter;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    source: PathBuf,
    target: Option<PathBuf>,

    // Replace target if already exists
    #[arg(long)]
    replace: bool,
}

impl Args {
    pub fn run() -> Result<()> {
        let args = Self::parse();

        if !args.source.exists() {
            anyhow::bail!("source path not found: '{}'", args.source.display());
        }

        if !args.source.is_dir() {
            anyhow::bail!(
                "expected a directory, found file: '{}'",
                args.source.display()
            );
        }

        let target = resolve_target_path(&args.source, args.target.as_deref())?;

        if target.exists() && !args.replace {
            anyhow::bail!(
                "target archive already exists: '{}'\nUse --replace to overwrite the existing archive.",
                target.display()
            );
        }

        let mut arc_writer = ArchiveWriter::new(&target)?;

        Self::add_source_to_archive(&mut arc_writer, &args.source, &target)?;

        Ok(())
    }

    fn add_source_to_archive(
        arc_writer: &mut ArchiveWriter,
        source: &Path,
        target: &Path,
    ) -> Result<()> {
        let mut file_count = 0;

        for entry in WalkDir::new(source) {
            let entry = entry
                .with_context(|| format!("failed to read entry inside '{}'", source.display()))?;

            if !entry.file_type().is_file() {
                continue;
            }

            let file_path = entry.path();

            arc_writer.add_file(file_path).with_context(|| {
                format!("failed to add file to archive: '{}'", file_path.display())
            })?;

            arc_writer.finish()?;

            file_count += 1;

            println!("a {}", file_path.display());
        }

        println!(
            "\nCreated archive '{}' with {} file{}.",
            target.display(),
            file_count,
            if file_count == 1 { "" } else { "s" }
        );

        Ok(())
    }
}

fn resolve_target_path<'a>(source: &'a Path, target: Option<&'a Path>) -> Result<Cow<'a, Path>> {
    match target {
        Some(target) => Ok(Cow::Borrowed(target)),
        None => {
            let file_name = source.file_name().map(Path::new).ok_or_else(|| {
                anyhow::anyhow!(
                    "could not get file name from source path: '{}'",
                    source.display()
                )
            })?;

            Ok(Cow::Owned(file_name.with_extension("oxa")))
        }
    }
}
