use std::result;

use clap::Parser;
use fs_rs::{cli::Cli, search::{FileSearcher, SearchResult}, filter::{FilenameFilter, SearchFilter}};


fn main() {
    let cli = Cli::parse();
    
    let filters = create_filters(&cli);
    let searcher = FileSearcher::new(filters);

    let paths = cli.paths.split('|').into_iter().collect::<Vec<&str>>();
    let results = searcher.search_paths(&paths);
    
    for result in results {
        match result {
            SearchResult::Directory(x) => println!("Dir: {:?}", x),
            SearchResult::File(x) => println!("File: {:?}", x),
            SearchResult::SymLink(x) => println!("SymLink: {:?}", x),
        }
    }
}

fn create_filters(cli: &Cli) -> Vec<Box<dyn SearchFilter>> {
    let names = cli.name_filter.split('|').into_iter().collect::<Vec<&str>>();
    let name_filter = FilenameFilter::new(&names);

    let mut filters: Vec<Box<dyn SearchFilter>> = Vec::new();
    filters.push(Box::new(name_filter));
    filters
}
