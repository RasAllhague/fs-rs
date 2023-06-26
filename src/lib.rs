pub mod cli;
pub mod filter;
pub mod search;

pub enum SearchError {
    WalkDir(walkdir::Error),
    IO(std::io::Error),
}
