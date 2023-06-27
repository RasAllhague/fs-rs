use std::fs;

use walkdir::DirEntry;

use crate::cli::{MatchOption, ShowResults};

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
    result_type: ShowResults,
}

impl FilenameFilter {
    pub fn new(file_names: &[&str], match_option: MatchOption, case_sensitiv: bool) -> Self {
        let file_names = file_names.iter().map(|x| x.to_string()).collect();

        Self {
            file_names,
            case_sensitiv,
            match_option,
        }
    }
}

impl SearchFilter for FilenameFilter {
    fn check_filter(&self, dir_entry: &DirEntry) -> bool {
        let name = dir_entry.file_name().to_str();

        match name {
            Some(n) => match self.case_sensitiv {
                true => match self.match_option {
                    MatchOption::All => self.file_names.iter().all(|x| n.contains(x)),
                    MatchOption::Any => self.file_names.iter().any(|x| n.contains(x)),
                    MatchOption::None => !self.file_names.iter().any(|x| n.contains(x)),
                },
                false => match self.match_option {
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
                },
            },
            None => false,
        }
    }
}

impl FileContentFilter {
    pub fn new(words: &[&str], match_option: MatchOption, case_sensitiv: bool) -> Self {
        let words = words.iter().map(|x| x.to_string()).collect();

        Self {
            words,
            case_sensitiv,
            match_option,
        }
    }

    fn check_content(&self, file_path: &str) -> bool {
        match self.case_sensitiv {
            true => match fs::read_to_string(file_path) {
                Ok(c) => match self.match_option {
                    MatchOption::All => self.words.iter().all(|x| c.contains(x)),
                    MatchOption::Any => self.words.iter().any(|x| c.contains(x)),
                    MatchOption::None => !self.words.iter().any(|x| c.contains(x)),
                },
                Err(_) => false,
            },
            false => match fs::read_to_string(file_path) {
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
            },
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
    pub fn new(entry_type: ShowResults) -> Self {
        Self {
            result_type: entry_type,
        }
    }
}

impl SearchFilter for EntryTypeFilter {
    fn check_filter(&self, dir_entry: &DirEntry) -> bool {
        match self.result_type {
            ShowResults::All => true,
            ShowResults::Directory => dir_entry.file_type().is_dir(),
            ShowResults::File => dir_entry.file_type().is_file(),
            ShowResults::SymLink => dir_entry.file_type().is_symlink(),
        }
    }
}
