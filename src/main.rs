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
    let (results, duration) = run_search(filters, &cli);
    display_results(results, duration, cli.max_results);
}

fn display_results(results: Vec<SearchResult>, duration: std::time::Duration, max_results: usize) {
    println!(
        "Displaying max {} out of {} results:",
        max_results,
        results.len()
    );

    for (i, result) in results.iter().take(max_results).enumerate() {
        match result {
            SearchResult::Directory {
                path,
                name,
                metadata: _,
            } => println!("({i}) Dir: {:?}, path: {:?}", name, path),
            SearchResult::File {
                path,
                name,
                metadata: _,
            } => println!("({i}) File: {:?}, path: {:?}", name, path),
            SearchResult::SymLink {
                path,
                name,
                metadata: _,
            } => println!("({i}) SymLink: {:?}, path: {:?}", name, path),
        }
    }

    println!(
        "Needed {}s for finding '{}' results.",
        duration.as_secs(),
        results.len()
    );
}

fn run_search(
    filters: Vec<Box<dyn SearchFilter>>,
    cli: &Cli,
) -> (Vec<SearchResult>, std::time::Duration) {
    let searcher = FileSearcher::new(filters, cli.depth);

    println!("Searching...");
    
    let paths: Vec<&str> = cli.search_paths.iter().map(|x| x.as_str()).collect();
    
    let start = Instant::now();
    let results = searcher.search_paths(&paths);
    let duration = start.elapsed();

    println!("Finished searching...");
    
    (results, duration)
}

fn create_filters(cli: &Cli) -> Vec<Box<dyn SearchFilter>> {
    let mut filters: Vec<Box<dyn SearchFilter>> = Vec::new();

    if let Some(names) = cli.names.clone() {
        let names: Vec<&str> = names.iter().map(|x| x.as_str()).collect();
        let name_filter = FilenameFilter::new(&names);
        filters.push(Box::new(name_filter));
    }

    if let Some(words) = cli.words.clone() {
        let words: Vec<&str> = words.iter().map(|x| x.as_str()).collect();
        let file_content_filter = FileContentFilter::new(&words);
        filters.push(Box::new(file_content_filter));
    }

    filters
}
