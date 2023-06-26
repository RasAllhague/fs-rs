use std::time::Instant;

use clap::Parser;
use fs_rs::{
    cli::Cli,
    filter::{FileContentFilter, FilenameFilter, SearchFilter},
    search::{FileSearcher, SearchResult},
};

fn main() {
    let cli = Cli::parse();

    run(cli);
}

fn run(cli: Cli) {
    let filters = create_filters(&cli);
    let (results, duration) = run_search(filters, cli);
    display_results(results, duration);
}

fn display_results(results: Vec<SearchResult>, duration: std::time::Duration) {
    for result in results.clone() {
        match result {
            SearchResult::Directory {
                path,
                name,
                metadata: _,
            } => println!("Dir: {:?}, path: {:?}", name, path),
            SearchResult::File {
                path,
                name,
                metadata: _,
            } => println!("File: {:?}, path: {:?}", name, path),
            SearchResult::SymLink {
                path,
                name,
                metadata: _,
            } => println!("SymLink: {:?}, path: {:?}", name, path),
        }
    }

    println!(
        "Needed {}s for finding '{}' items.",
        duration.as_secs(),
        results.len()
    );
}

fn run_search(
    filters: Vec<Box<dyn SearchFilter>>,
    cli: Cli,
) -> (Vec<SearchResult>, std::time::Duration) {
    let searcher = FileSearcher::new(filters, cli.depth);

    let paths = cli
        .search_paths
        .split('|')
        .into_iter()
        .collect::<Vec<&str>>();

    let start = Instant::now();
    let results = searcher.search_paths(&paths);
    let duration = start.elapsed();
    (results, duration)
}

fn create_filters(cli: &Cli) -> Vec<Box<dyn SearchFilter>> {
    let mut filters: Vec<Box<dyn SearchFilter>> = Vec::new();

    if let Some(names) = cli.names.clone() {
        let names = names.split('|').into_iter().collect::<Vec<&str>>();
        let name_filter = FilenameFilter::new(&names);
        filters.push(Box::new(name_filter));
    }

    if let Some(words) = cli.words.clone() {
        let words = words.split('|').into_iter().collect::<Vec<&str>>();
        let file_content_filter = FileContentFilter::new(&words);
        filters.push(Box::new(file_content_filter));
    }

    filters
}
