use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::utils::inferred_target;
use crate::writer::ArchiveWriter;

#[derive(Debug, Subcommand)]
enum Commands {
    Pack(PackArgs),
}

#[derive(Args, Debug)]
struct PackArgs {
    source: PathBuf,
    target: Option<PathBuf>,

    #[arg(short, long)]
    replace: bool,
}

impl PackArgs {
    fn execute(&self) -> Result<()> {
        if !self.source.exists() {
            anyhow::bail!("source path not found: '{}'", self.source.display());
        }

        if !self.source.is_dir() {
            anyhow::bail!(
                "expected a directory, found file: '{}'",
                self.source.display()
            );
        }
        let target = inferred_target(&self.source, self.target.as_deref())?;

        if target.exists() && !self.replace {
            anyhow::bail!(
                "target archive already exists: '{}'\nUse --replace to overwrite the existing archive.",
                target.display()
            );
        }

        let mut arc_writer = ArchiveWriter::new(&target)?;

        Self::add_source_to_archive(&mut arc_writer, &self.source, &target)?;

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

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    commands: Commands,
}

impl CliArgs {
    pub fn run() -> Result<()> {
        let args = Self::parse();
        match args.commands {
            Commands::Pack(cmd) => cmd.execute()?,
        }

        Ok(())
    }
}
