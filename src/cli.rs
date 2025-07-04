use std::fmt::Display;

use clap::{command, Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(short, long)]
    pub search_paths: Vec<String>,
    #[arg(short, long, default_value_t = 10000)]
    pub depth: usize,
    #[arg(short, long, default_value_t = 10)]
    pub max_results: usize,
    #[command(subcommand)]
    pub search: Option<Search>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Search {
    Name(SearchTypeArgs),
    Content(SearchTypeArgs),
}

#[derive(Debug, Args, Clone)]
pub struct SearchTypeArgs {
    #[arg(short, long)]
    pub names: Vec<String>,
    #[arg(short, long, default_value_t = true)]
    pub case_sensisitiv: bool,
    #[arg(short, long, default_value_t = MatchOption::Any)]
    pub match_option: MatchOption,
    #[arg(short, long)]
    pub order_by: Option<OrderBy>,
    #[arg(short, long, default_value_t = ResultFilter::All)]
    pub result_filter: ResultFilter,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum MatchOption {
    All,
    Any,
    None,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrderBy {
    None,
    Name,
    Path,
    Size,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ResultFilter {
    All,
    Directory,
    File,
    SymLink,
}

impl Display for MatchOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatchOption::All => write!(f, "all"),
            MatchOption::Any => write!(f, "any"),
            MatchOption::None => write!(f, "none"),
        }
    }
}

impl Display for ResultFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResultFilter::All => write!(f, "all"),
            ResultFilter::Directory => write!(f, "directory"),
            ResultFilter::File => write!(f, "file"),
            ResultFilter::SymLink => write!(f, "symlink"),
        }
    }
}
