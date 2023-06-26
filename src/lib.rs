pub mod search;
pub mod cli;
pub mod filter;

pub enum SearchError {
    WalkDir(walkdir::Error),
    IO(std::io::Error),
}