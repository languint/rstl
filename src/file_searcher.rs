use crate::errors::Errors;
use color_eyre::eyre::{Result, WrapErr};
use std::fs;
use std::path::PathBuf;

pub struct FileSearcher {
    target_dir: PathBuf,
}

pub type SearchResult = Vec<PathBuf>;

impl FileSearcher {
    pub fn new(target_dir: impl Into<PathBuf>) -> Self {
        FileSearcher {
            target_dir: target_dir.into(),
        }
    }

    pub fn has_suffix(file_name: &str, suffix: &str) -> bool {
        file_name.ends_with(suffix)
    }

    pub fn search(&self, suffix: &str) -> Result<SearchResult> {
        let mut files = Vec::new();

        let read_dir = fs::read_dir(&self.target_dir)
            .wrap_err_with(|| Errors::ReadDirError(format!("{}", self.target_dir.display())))?;

        for entry_result in read_dir {
            let entry = entry_result.with_context(|| {
                format!(
                    "Failed to read directory entry in {}",
                    self.target_dir.display()
                )
            })?;

            let path = entry.path();

            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if FileSearcher::has_suffix(name, suffix) {
                    files.push(path);
                }
            }
        }

        Ok(files)
    }
}
