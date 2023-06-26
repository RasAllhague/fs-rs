use std::ffi::OsString;

use walkdir::{DirEntry, WalkDir};

use crate::filter::SearchFilter;

pub struct FileSearcher {
    filters: Vec<Box<dyn SearchFilter>>,
}

impl FileSearcher {
    pub fn new(filters: Vec<Box<dyn SearchFilter>>) -> Self {
        Self { filters }
    }

    pub fn search_paths(&self, paths: &[&str]) -> Vec<SearchResult> {
        paths.iter().flat_map(|x| self.search_path(x)).collect()
    }

    fn search_path(&self, path: &str) -> Vec<SearchResult> {
        WalkDir::new(path)
            .into_iter()
            .filter(|x| x.is_ok())
            .map(|e| e.unwrap())
            .filter(|e| self.check_filters(e))
            .map(|e| map_filetype(e))
            .collect()
    }

    fn check_filters(&self, dir_entry: &DirEntry) -> bool {
        for filter in self.filters.iter() {
            if filter.check_filter(dir_entry) {
                return true;
            }
        }

        return false;
    }
}

fn map_filetype(dir_entry: DirEntry) -> SearchResult {
    if dir_entry.file_type().is_file() {
        return SearchResult::File(dir_entry.file_name().to_os_string());
    }

    if dir_entry.file_type().is_dir() {
        return SearchResult::Directory(dir_entry.file_name().to_os_string());
    }

    SearchResult::SymLink(dir_entry.file_name().to_os_string())
}

pub enum SearchResult {
    Directory(OsString),
    File(OsString),
    SymLink(OsString),
}
