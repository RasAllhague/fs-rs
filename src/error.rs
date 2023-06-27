use inquire::InquireError;

#[derive(Debug)]
pub enum FsRsError {
    SearchError(SearchError),
    Inquire(InquireError),
    Crossterm(std::io::Error),
}

#[derive(Debug)]
pub enum SearchError {
    WalkDir(walkdir::Error),
    IO(std::io::Error),
}

impl From<walkdir::Error> for SearchError {
    fn from(value: walkdir::Error) -> Self {
        SearchError::WalkDir(value)
    }
}

impl From<std::io::Error> for SearchError {
    fn from(value: std::io::Error) -> Self {
        SearchError::IO(value)
    }
}

impl From<std::io::Error> for FsRsError {
    fn from(value: std::io::Error) -> Self {
        FsRsError::Crossterm(value)
    }
}

impl From<SearchError> for FsRsError {
    fn from(value: SearchError) -> Self {
        FsRsError::SearchError(value)
    }
}

impl From<InquireError> for FsRsError {
    fn from(value: InquireError) -> Self {
        FsRsError::Inquire(value)
    }
}