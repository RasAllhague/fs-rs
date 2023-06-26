use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(short, long)]
    pub search_paths: Vec<String>,
    #[arg(short, long)]
    pub names: Option<Vec<String>>,
    #[arg(short, long, default_value_t = 10000)]
    pub depth: usize,
    #[arg(short, long)]
    pub words: Option<Vec<String>>,
    #[arg(short, long, default_value_t = 20)]
    pub max_results: usize,
}
