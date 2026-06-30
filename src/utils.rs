use std::{borrow::Cow, path::Path};

use anyhow::Result;

/// Makes a target path with `.oxa` extension from source.
///
/// If target is provided this function returns it as Cow::Borrowed
/// otherwise uses source folder name to construct and return the target
/// path as Cow::Owned.
pub(crate) fn inferred_target<'a>(
    source: &'a Path,
    target: Option<&'a Path>,
) -> Result<Cow<'a, Path>> {
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
