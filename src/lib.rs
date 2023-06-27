pub mod cli;
pub mod filter;
pub mod search;

#[derive(Debug)]
pub enum SearchError {
    WalkDir(walkdir::Error),
    IO(std::io::Error),
}
