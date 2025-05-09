use std::fs;
use std::path::PathBuf;

use crate::util::print_warning;

pub struct FileSearcher {
    target_dir: PathBuf,
}

pub type SearchResult = Vec<String>;

impl FileSearcher {
    pub fn new(target_dir: impl Into<PathBuf>) -> Self {
        FileSearcher {
            target_dir: target_dir.into(),
        }
    }

    pub fn has_suffix(file_name: &str, suffix: &str) -> bool {
        file_name.ends_with(suffix)
    }

    pub fn search(&self, suffix: &str) -> SearchResult {
        let mut files: SearchResult = Vec::new();

        let read_dir = fs::read_dir(&self.target_dir);

        let entries = match read_dir {
            Ok(entries) => entries,
            Err(_) => {
                print_warning(
                    &format!("Failed to read dir {}", self.target_dir.display()),
                    0,
                );
                return files;
            }
        };

        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(name) = entry.file_name().to_str() {
                    if FileSearcher::has_suffix(name, suffix) {
                        files.push(name.to_string());
                    }
                }
            }
        }

        files
    }
}
