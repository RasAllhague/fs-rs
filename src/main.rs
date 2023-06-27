use std::{fs, io::stdout, time::Instant};

use clap::Parser;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use fs_rs::{
    cli::Cli,
    filter::{FileContentFilter, FilenameFilter, SearchFilter},
    search::{FileSearcher, SearchResult},
    SearchError,
};
use inquire::{Confirm, InquireError, MultiSelect, Select};

static OPEN_ENTRY: &str = "Open entries";
static SHOW_DETAILS: &str = "Show details";
static COPY_ENTRIES: &str = "Copy entries";
static MOVE_ENTRIES: &str = "Move entries";
static DELETE_ENTRIES: &str = "Delete entries";

#[derive(Debug)]
pub enum FsRsError {
    SearchError(SearchError),
    Inquire(InquireError),
    Crossterm(std::io::Error),
}

fn main() -> Result<(), FsRsError> {
    let cli = Cli::parse();

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
        SHOW_DETAILS,
        COPY_ENTRIES,
        MOVE_ENTRIES,
        DELETE_ENTRIES,
    ];

    let entry_action = Select::new("What do you want to do?", options)
        .prompt()
        .map_err(|x| FsRsError::Inquire(x))?;

    match entry_action {
        "Open entries" => open_files(results, max_results),
        "Show details" => show_details(results, max_results),
        "Copy entries" => copy_entries(results, max_results),
        "Move entries" => move_entries(results, max_results),
        "Delete entries" => delete_entries(results, max_results),
        _ => print_error("Invalid option entered!"),
    }
}

fn open_files(results: Vec<SearchResult>, max_results: usize) -> Result<(), FsRsError> {
    let selected = MultiSelect::new("Which entries do you want to open?", results)
        .with_page_size(max_results)
        .prompt_skippable()
        .map_err(|x| FsRsError::Inquire(x))?;

    if let Some(entries) = selected {
        for entry in entries.iter() {
            print_search_result(entry)?;
        }
    }

    Ok(())
}

fn show_details(results: Vec<SearchResult>, max_results: usize) -> Result<(), FsRsError> {
    let selected = MultiSelect::new("Which entries do you want to see details from?", results)
        .with_page_size(max_results)
        .prompt_skippable()
        .map_err(|x| FsRsError::Inquire(x))?;

    if let Some(entries) = selected {
        for entry in entries.iter() {
            print_search_result(entry)?;
        }
    }

    Ok(())
}

fn copy_entries(results: Vec<SearchResult>, max_results: usize) -> Result<(), FsRsError> {
    let selected = MultiSelect::new("Which entries do you want to copy?", results)
        .with_page_size(max_results)
        .prompt_skippable()
        .map_err(|x| FsRsError::Inquire(x))?;

    if let Some(entries) = selected {
        for entry in entries.iter() {
            print_search_result(entry)?;
        }
    }

    Ok(())
}

fn move_entries(results: Vec<SearchResult>, max_results: usize) -> Result<(), FsRsError> {
    let selected = MultiSelect::new("Which entries do you want to move?", results)
        .with_page_size(max_results)
        .prompt_skippable()
        .map_err(|x| FsRsError::Inquire(x))?;

    if let Some(entries) = selected {
        for entry in entries.iter() {
            print_search_result(entry)?;
        }
    }

    Ok(())
}

fn delete_entries(results: Vec<SearchResult>, max_results: usize) -> Result<(), FsRsError> {
    let selected = MultiSelect::new("Which entries do you want to see delete?", results)
        .with_page_size(max_results)
        .prompt_skippable()
        .map_err(|x| FsRsError::Inquire(x))?;

    if let Some(entries) = selected {
        for entry in entries.iter() {
            print_warning(&format!("Attempting to delete: {:?}!", entry.path()))?;
            let confirmation = Confirm::new("Are you sure?")
                .with_default(false)
                .prompt()
                .map_err(|x| FsRsError::Inquire(x))?;

            if confirmation {
                match entry {
                    SearchResult::Directory {
                        path,
                        name: _,
                        metadata: _,
                    } => fs::remove_dir_all(path).map_err(|x| FsRsError::Crossterm(x)),
                    SearchResult::File {
                        path,
                        name: _,
                        metadata: _,
                    } => fs::remove_file(path).map_err(|x| FsRsError::Crossterm(x)),
                    SearchResult::SymLink {
                        path,
                        name: _,
                        metadata: _,
                    } => fs::remove_file(path).map_err(|x| FsRsError::Crossterm(x)),
                }?;
            }
        }
    }

    Ok(())
}

fn print_error(message: &str) -> Result<(), FsRsError> {
    print_log(message, Color::Red)
}

fn print_warning(message: &str) -> Result<(), FsRsError> {
    print_log(message, Color::Yellow)
}

fn print_log(message: &str, color: Color) -> Result<(), FsRsError> {
    execute!(
        stdout(),
        SetForegroundColor(color),
        Print(message),
        Print("\n"),
        ResetColor
    )
    .map_err(|x| FsRsError::Crossterm(x))
}

fn print_message(message: &str) -> Result<(), FsRsError> {
    print_log(message, Color::Grey)
}

fn print_search_result(search_result: &SearchResult) -> Result<(), FsRsError> {
    let message = match search_result {
        SearchResult::Directory {
            path,
            name: _,
            metadata: _,
        } => format!("(D) Opening: {:?}", path),
        SearchResult::File {
            path,
            name: _,
            metadata: _,
        } => format!("(F) Opening: {:?}", path),
        SearchResult::SymLink {
            path,
            name: _,
            metadata: _,
        } => format!("(S) Opening: {:?}", path),
    };

    print_message(&message)
}
