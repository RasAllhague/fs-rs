use std::fs;

use walkdir::DirEntry;

use crate::cli::{MatchOption, ResultFilter};

pub trait SearchFilter {
    fn check_filter(&self, dir_entry: &DirEntry) -> bool;
}

pub struct FilenameFilter {
    file_names: Vec<String>,
    match_option: MatchOption,
    case_sensitiv: bool,
}

pub struct FileContentFilter {
    words: Vec<String>,
    match_option: MatchOption,
    case_sensitiv: bool,
}

pub struct EntryTypeFilter {
    result_type: ResultFilter,
}

impl FilenameFilter {
    pub fn new(file_names: &[&str], match_option: MatchOption, case_sensitiv: bool) -> Self {
        let file_names = file_names.iter().map(ToString::to_string).collect();

        Self {
            file_names,
            match_option,
            case_sensitiv,
        }
    }
}

impl SearchFilter for FilenameFilter {
    fn check_filter(&self, dir_entry: &DirEntry) -> bool {
        let name = dir_entry.file_name().to_str();

        match name {
            Some(n) => {
                if self.case_sensitiv {
                    match self.match_option {
                        MatchOption::All => self.file_names.iter().all(|x| n.contains(x)),
                        MatchOption::Any => self.file_names.iter().any(|x| n.contains(x)),
                        MatchOption::None => !self.file_names.iter().any(|x| n.contains(x)),
                    }
                } else {
                    match self.match_option {
                        MatchOption::All => self.file_names.iter().all(|x| {
                            n.to_lowercase()
                                .as_str()
                                .contains(x.to_lowercase().as_str())
                        }),
                        MatchOption::Any => self.file_names.iter().any(|x| {
                            n.to_lowercase()
                                .as_str()
                                .contains(x.to_lowercase().as_str())
                        }),
                        MatchOption::None => !self.file_names.iter().any(|x| {
                            n.to_lowercase()
                                .as_str()
                                .contains(x.to_lowercase().as_str())
                        }),
                    }
                }
            }
            None => false,
        }
    }
}

impl FileContentFilter {
    pub fn new(words: &[&str], match_option: MatchOption, case_sensitiv: bool) -> Self {
        let words = words.iter().map(ToString::to_string).collect();

        Self {
            words,
            match_option,
            case_sensitiv,
        }
    }

    fn check_content(&self, file_path: &str) -> bool {
        if self.case_sensitiv {
            match fs::read_to_string(file_path) {
                Ok(c) => match self.match_option {
                    MatchOption::All => self.words.iter().all(|x| c.contains(x)),
                    MatchOption::Any => self.words.iter().any(|x| c.contains(x)),
                    MatchOption::None => !self.words.iter().any(|x| c.contains(x)),
                },
                Err(_) => false,
            }
        } else {
            match fs::read_to_string(file_path) {
                Ok(c) => match self.match_option {
                    MatchOption::All => self.words.iter().all(|x| {
                        c.to_lowercase()
                            .as_str()
                            .contains(x.to_lowercase().as_str())
                    }),
                    MatchOption::Any => self.words.iter().any(|x| {
                        c.to_lowercase()
                            .as_str()
                            .contains(x.to_lowercase().as_str())
                    }),
                    MatchOption::None => !self.words.iter().any(|x| {
                        c.to_lowercase()
                            .as_str()
                            .contains(x.to_lowercase().as_str())
                    }),
                },
                Err(_) => false,
            }
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

impl EntryTypeFilter {
    #[must_use]
    pub fn new(entry_type: ResultFilter) -> Self {
        Self {
            result_type: entry_type,
        }
    }
}

impl SearchFilter for EntryTypeFilter {
    fn check_filter(&self, dir_entry: &DirEntry) -> bool {
        match self.result_type {
            ResultFilter::All => true,
            ResultFilter::Directory => dir_entry.file_type().is_dir(),
            ResultFilter::File => dir_entry.file_type().is_file(),
            ResultFilter::SymLink => dir_entry.file_type().is_symlink(),
        }
    }
}
