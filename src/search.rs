use std::{ffi::OsString, fmt::Display, fs::Metadata};

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
    #[must_use]
    pub fn new(filters: Vec<Box<dyn SearchFilter>>, max_depth: usize) -> Self {
        Self { filters, max_depth }
    }

    #[must_use]
    pub fn search_paths(&self, paths: &[&str]) -> Vec<SearchResult> {
        paths.iter().flat_map(|x| self.search_path(x)).collect()
    }

    fn search_path(&self, path: &str) -> Vec<SearchResult> {
        WalkDir::new(path)
            .max_depth(self.max_depth)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| self.check_filters(e))
            .map(|x| map_filetype(&x))
            .collect()
    }

    fn check_filters(&self, dir_entry: &DirEntry) -> bool {
        if self.filters.is_empty() {
            return true;
        }

        let mut filter_result = true;

        for filter in &self.filters {
            if !filter.check_filter(dir_entry) {
                filter_result = false;
            }
        }

        filter_result
    }
}

fn map_filetype(dir_entry: &DirEntry) -> SearchResult {
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

impl SearchResult {
    #[must_use]
    pub fn path(&self) -> OsString {
        match self {
            SearchResult::Directory {
                path,
                name: _,
                metadata: _,
            } | SearchResult::File {
                path,
                name: _,
                metadata: _,
            } | SearchResult::SymLink {
                path,
                name: _,
                metadata: _,
            } => path.clone(),
        }
    }
}

impl Display for SearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchResult::Directory {
                path,
                name,
                metadata: _,
            } => write!(
                f,
                "(D): {:?}, path: {:?}",
                name,
                truncate(path.to_str().unwrap_or("Not parsable"), 50)
            ),
            SearchResult::File {
                path,
                name,
                metadata: _,
            } => write!(
                f,
                "(f): {:?}, path: {:?}",
                name,
                truncate(path.to_str().unwrap_or("Not parsable"), 50)
            ),
            SearchResult::SymLink {
                path,
                name,
                metadata: _,
            } => write!(
                f,
                "(s): {:?}, path: {:?}",
                name,
                truncate(path.to_str().unwrap_or("Not parsable"), 50)
            ),
        }
    }
}

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}
