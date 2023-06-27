use std::time::Instant;

use clap::Parser;

use fs_rs::{
    cli::{Cli, Search},
    filter::{FileContentFilter, FilenameFilter, SearchFilter, EntryTypeFilter},
    search::{FileSearcher, SearchResult}, error::FsRsError, displaying::{print_message, print_error}, dialogue::{DeleteEntriesDialogue, MoveEntriesDialogue, CopyEntriesDialogue, ShowEntriesDialogue, OpenEntriesDialogue, RevealEntriesDialogue},
};
use inquire::{Select, Confirm};

static OPEN_ENTRY: &str = "Open entries";
static SHOW_DETAILS: &str = "Show details";
static COPY_ENTRIES: &str = "Copy entries";
static REVEAL_ENTRIES: &str = "Reveal entries";
static MOVE_ENTRIES: &str = "Move entries";
static DELETE_ENTRIES: &str = "Delete entries";



fn main() -> Result<(), FsRsError> {
    let cli = Cli::parse();

    if cli.search_paths.len() == 0 {
        print_error("You have to provide atleast one search path!")?;
        return Ok(());
    }

    run(cli)
}

fn run(cli: Cli) -> Result<(), FsRsError> {
    let filters = create_filters(&cli);
    let (results, duration) = run_search(filters, &cli)?;

    display_results(results, duration, cli.max_results)
}

fn run_search(
    filters: Vec<Box<dyn SearchFilter>>,
    cli: &Cli,
) -> Result<(Vec<SearchResult>, std::time::Duration), FsRsError> {
    let searcher = FileSearcher::new(filters, cli.depth);

    print_message("Searching...")?;

    let paths: Vec<&str> = cli.search_paths.iter().map(|x| x.as_str()).collect();

    let start = Instant::now();
    let results = searcher.search_paths(&paths);
    let duration = start.elapsed();

    print_message("Finished searching...")?;

    Ok((results, duration))
}

fn create_filters(cli: &Cli) -> Vec<Box<dyn SearchFilter>> {
    let mut filters: Vec<Box<dyn SearchFilter>> = Vec::new();

    if let Some(search_v) = &cli.search {
        let search_filters: (Box<dyn SearchFilter>, Box<dyn SearchFilter>) = match search_v {
            Search::Name(args) => {
                let names: Vec<&str> = args.names.iter().map(|x| x.as_str()).collect();
                let name_filter = FilenameFilter::new(&names, args.match_option, args.case_sensisitiv);
                let result_type_filter = EntryTypeFilter::new(args.show_results);

                (Box::new(name_filter), Box::new(result_type_filter))
            }
            Search::Content(args) => {
                let words: Vec<&str> = args.names.iter().map(|x| x.as_str()).collect();
                let file_content_filter = FileContentFilter::new(&words, args.match_option, args.case_sensisitiv);
                let result_type_filter = EntryTypeFilter::new(args.show_results);

                (Box::new(file_content_filter), Box::new(result_type_filter))
            },
        };

        filters.push(search_filters.0);
        filters.push(search_filters.1);
    } 

    filters
}

fn display_results(
    results: Vec<SearchResult>,
    duration: std::time::Duration,
    max_results: usize,
) -> Result<(), FsRsError> {
    print_message(&format!(
        "Needed {}s for finding '{}' results.",
        duration.as_secs(),
        results.len()
    ))?;

    let options = vec![
        OPEN_ENTRY,
        REVEAL_ENTRIES,
        SHOW_DETAILS,
        COPY_ENTRIES,
        MOVE_ENTRIES,
        DELETE_ENTRIES,
    ];

    loop {
        let entry_action = Select::new("What do you want to do?", options.clone())
        .prompt_skippable()?;
    
        if let Some(action) = entry_action {
            match action {
                "Open entries" => OpenEntriesDialogue::show(&results, max_results),
                "Reveal entries" => RevealEntriesDialogue::show(&results, max_results),
                "Show details" => ShowEntriesDialogue::show(&results, max_results),
                "Copy entries" => CopyEntriesDialogue::show(&results, max_results),
                "Move entries" => MoveEntriesDialogue::show(&results, max_results),
                "Delete entries" => DeleteEntriesDialogue::show(&results, max_results),
                _ => print_error("Invalid option entered!"),
            }?;
        }

        match Confirm::new("Do you want to do additional things?").with_default(true).prompt() {
            Ok(res) => if !res {
                return Ok(())
            },
            Err(_) => return Ok(()),
        };
    }
}