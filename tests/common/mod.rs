use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use tempfile::{NamedTempFile, TempDir};
use walkdir::WalkDir;

pub fn create_temp_file() -> (TempDir, NamedTempFile) {
    let dir = TempDir::new().unwrap();
    let file = NamedTempFile::new_in(&dir).unwrap();
    (dir, file)
}

pub fn collect_files<P, E>(path: P, extensions: &[E]) -> Vec<PathBuf>
where
    P: AsRef<Path>,
    E: AsRef<OsStr>,
{
    WalkDir::new(path)
        .into_iter()
        .filter_map(move |entry| {
            let path = entry.ok()?.into_path();
            let extension = path.extension()?;
            extensions.iter().find(|e| extension.eq_ignore_ascii_case(e))?;
            Some(path)
        })
        .collect()
}
