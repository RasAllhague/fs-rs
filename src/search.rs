use std::{ffi::OsString, fs::Metadata};

use walkdir::{DirEntry, WalkDir};

use crate::filter::SearchFilter;

pub enum SearchMode {
    TopLevelOnly,
    Recursive,
}

pub struct FileSearcher {
    filters: Vec<Box<dyn SearchFilter>>,
    max_depth: usize,
}

impl FileSearcher {
    pub fn new(filters: Vec<Box<dyn SearchFilter>>, max_depth: usize) -> Self {
        Self { filters, max_depth }
    }

    pub fn search_paths(&self, paths: &[&str]) -> Vec<SearchResult> {
        paths.iter().flat_map(|x| self.search_path(x)).collect()
    }

    fn search_path(&self, path: &str) -> Vec<SearchResult> {
        WalkDir::new(path)
            .max_depth(self.max_depth)
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
        return SearchResult::File {
            path: dir_entry.path().as_os_str().to_os_string(),
            name: dir_entry.file_name().to_os_string(),
            metadata: dir_entry.metadata().ok(),
        };
    }

    if dir_entry.file_type().is_dir() {
        return SearchResult::Directory {
            path: dir_entry.path().as_os_str().to_os_string(),
            name: dir_entry.file_name().to_os_string(),
            metadata: dir_entry.metadata().ok(),
        };
    }

    SearchResult::SymLink {
        path: dir_entry.path().as_os_str().to_os_string(),
        name: dir_entry.file_name().to_os_string(),
        metadata: dir_entry.metadata().ok(),
    }
}

#[derive(Clone, Debug)]
pub enum SearchResult {
    Directory {
        path: OsString,
        name: OsString,
        metadata: Option<Metadata>,
    },
    File {
        path: OsString,
        name: OsString,
        metadata: Option<Metadata>,
    },
    SymLink {
        path: OsString,
        name: OsString,
        metadata: Option<Metadata>,
    },
}
