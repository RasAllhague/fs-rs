use std::fs;

use walkdir::DirEntry;

pub enum MatchOptions {
    All,
    Any,
    Exact,
}

pub trait SearchFilter {
    fn check_filter(&self, dir_entry: &DirEntry) -> bool;
}

pub struct FilenameFilter {
    file_names: Vec<String>,
}

impl FilenameFilter {
    pub fn new(file_names: &[&str]) -> Self {
        let file_names = file_names.iter().map(|x| x.to_string()).collect();

        Self { file_names }
    }
}

impl SearchFilter for FilenameFilter {
    fn check_filter(&self, dir_entry: &DirEntry) -> bool {
        let name = dir_entry.file_name().to_str();

        match name {
            Some(n) => self.file_names.iter().any(|x| n.contains(x)),
            None => false,
        }
    }
}

pub struct FileContentFilter {
    words: Vec<String>,
}

impl FileContentFilter {
    pub fn new(words: &[&str]) -> Self {
        let words = words.iter().map(|x| x.to_string()).collect();

        Self { words}
    }

    fn check_content(&self, file_path: &str) -> bool {
        match fs::read_to_string(file_path) {
            Ok(c) => self.words.iter().any(|x| c.contains(x)),
            Err(_) => false,
        }
    }
}

impl SearchFilter for FileContentFilter {
    fn check_filter(&self, dir_entry: &DirEntry) -> bool {
        match dir_entry.path().to_str() {
            Some(f) => self.check_content(f),
            None => false,
        }
    }
}