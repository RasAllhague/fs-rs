use walkdir::DirEntry;

pub trait SearchFilter {
    fn check_filter(&self, dir_entry: &DirEntry) -> bool;
}

pub struct FilenameFilter {
    file_names: Vec<String>,
}

impl FilenameFilter {
    pub fn new(file_names: &[&str]) -> Self {
        let file_names = file_names.iter().map(|x| x.to_string()).collect();

        Self {
            file_names
        }
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