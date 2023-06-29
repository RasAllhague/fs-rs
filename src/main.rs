use std::time::Instant;

use clap::Parser;

use fs_rs::{
    cli::{Cli, MatchOption, ResultFilter, Search},
    dialogue::{
        CopyEntriesDialogue, DeleteEntriesDialogue, MoveEntriesDialogue, OpenEntriesDialogue,
        RevealEntriesDialogue, ShowEntriesDialogue,
    },
    displaying::{print_error, print_message, print_warning},
    error::FsRsError,
    filter::{EntryTypeFilter, FileContentFilter, FilenameFilter, SearchFilter},
    search::{FileSearcher, SearchResult},
};
use inquire::{Confirm, CustomType, Select, Text};

static OPEN_ENTRY: &str = "Open entries";
static SHOW_DETAILS: &str = "Show details";
static COPY_ENTRIES: &str = "Copy entries";
static REVEAL_ENTRIES: &str = "Reveal entries";
static MOVE_ENTRIES: &str = "Move entries";
static DELETE_ENTRIES: &str = "Delete entries";

fn main() -> Result<(), FsRsError> {
    let cli = Cli::parse();

    if cli.search_paths.is_empty() {
        return run_dialogue();
    }

    run_cli(&cli)
}

fn run_cli(cli: &Cli) -> Result<(), FsRsError> {
    let filters = create_filters_from_cli(cli);
    let (results, duration) = run_search(filters, &cli.search_paths, cli.depth)?;

    display_results(&results, duration, cli.max_results)
}

fn run_dialogue() -> Result<(), FsRsError> {
    let help = "Skip to continue.";
    let search_paths =
        show_multiple_inputs_dialogue("Which paths do you want to search in?", help)?;

    if search_paths.is_empty() {
        return print_warning("Search aborted!");
    }

    let (filenames, filename_match_option, filename_case_sensitiv) =
        show_filter_creation_dialogue("Which filenames do you search for?", "How do you want to match the filenames?", help)?;
    let (filecontents, filecontent_match_option, filecontent_case_sensitiv) =
        show_filter_creation_dialogue("Which filecontents do you search for?", "How do you want to match the filecontents?", help)?;

    let results_filter = show_results_filter_dialogue()?;

    let filters = create_filters_for_dialogue(
        filenames,
        filename_match_option,
        filename_case_sensitiv,
        filecontents,
        filecontent_match_option,
        filecontent_case_sensitiv,
        results_filter,
    );

    let max_depths = CustomType::<usize>::new("How deep do you want to search?")
        .with_default(1000)
        .prompt()?;
    let max_results = CustomType::<usize>::new("How many results do you want to see?")
        .with_default(10)
        .prompt()?;

    let (results, duration) = run_search(filters, &search_paths, max_depths)?;

    display_results(&results, duration, max_results)
}

fn show_filter_creation_dialogue(
    message: &str,
    option_message: &str,
    help: &str,
) -> Result<(Vec<String>, Option<MatchOption>, Option<bool>), FsRsError> {
    let search_words = show_multiple_inputs_dialogue(message, help)?;

    if search_words.is_empty() {
        return Ok((search_words, None, None));
    }

    let match_option = show_match_option_dialogue(option_message)?;
    let case_sensitiv = show_case_sensitiv_dialogue()?;
    Ok((search_words, Some(match_option), Some(case_sensitiv)))
}

fn show_multiple_inputs_dialogue(message: &str, help: &str) -> Result<Vec<String>, FsRsError> {
    let mut filenames = Vec::new();

    loop {
        let filename = Text::new(message)
            .with_help_message(help)
            .prompt_skippable()?;

        match filename {
            Some(f) => filenames.push(f),
            None => return Ok(filenames),
        }
    }
}

fn show_case_sensitiv_dialogue() -> Result<bool, FsRsError> {
    let case_sensitiv = Confirm::new("Do you want to search case sensitiv?").with_default(true).prompt()?;

    Ok(case_sensitiv)
}

fn show_results_filter_dialogue() -> Result<ResultFilter, FsRsError> {
    let options = vec!["All", "Directory", "File", "Symlink"];
    
    let result_filter = Select::new("What filesystem entries do you want to display?", options).with_starting_cursor(0).prompt()?;

    match result_filter {
        "Directory" => Ok(ResultFilter::Directory),
        "File" => Ok(ResultFilter::File),
        "Symlink" => Ok(ResultFilter::SymLink),
        _ => Ok(ResultFilter::All),
    }
}

fn show_match_option_dialogue(message: &str) -> Result<MatchOption, FsRsError> {
    let options = vec!["All", "Any", "None"];

    let match_option = Select::new(message, options).with_starting_cursor(1).prompt()?;

    match match_option {
        "All" => Ok(MatchOption::All),
        "None" => Ok(MatchOption::None),
        _ => Ok(MatchOption::Any),
    } 
}

fn run_search(
    filters: Vec<Box<dyn SearchFilter>>,
    search_paths: &[String],
    depth: usize,
) -> Result<(Vec<SearchResult>, std::time::Duration), FsRsError> {
    let searcher = FileSearcher::new(filters, depth);

    print_message("Searching...")?;

    let paths: Vec<&str> = search_paths
        .iter()
        .map(std::string::String::as_str)
        .collect();

    let start = Instant::now();
    let results = searcher.search_paths(&paths);
    let duration = start.elapsed();

    print_message("Finished searching...")?;

    Ok((results, duration))
}

fn create_filters_from_cli(cli: &Cli) -> Vec<Box<dyn SearchFilter>> {
    let mut filters: Vec<Box<dyn SearchFilter>> = Vec::new();

    if let Some(search_v) = &cli.search {
        let search_filters: (Box<dyn SearchFilter>, Box<dyn SearchFilter>) = match search_v {
            Search::Name(args) => {
                let names: Vec<&str> = args.names.iter().map(std::string::String::as_str).collect();
                let name_filter =
                    FilenameFilter::new(&names, args.match_option, args.case_sensisitiv);
                let result_type_filter = EntryTypeFilter::new(args.result_filter);

                (Box::new(name_filter), Box::new(result_type_filter))
            }
            Search::Content(args) => {
                let words: Vec<&str> = args.names.iter().map(std::string::String::as_str).collect();
                let file_content_filter =
                    FileContentFilter::new(&words, args.match_option, args.case_sensisitiv);
                let result_type_filter = EntryTypeFilter::new(args.result_filter);

                (Box::new(file_content_filter), Box::new(result_type_filter))
            }
        };

        filters.push(search_filters.0);
        filters.push(search_filters.1);
    }

    filters
}

fn create_filters_for_dialogue(
    filenames: Vec<String>,
    filename_match_option: Option<MatchOption>,
    filename_case_sensitiv: Option<bool>,
    filecontents: Vec<String>,
    filecontent_match_option: Option<MatchOption>,
    filecontent_case_sensitiv: Option<bool>,
    results_filter: ResultFilter,
) -> Vec<Box<dyn SearchFilter>> {
    let mut filters: Vec<Box<dyn SearchFilter>> = Vec::new();

    if !filenames.is_empty() {
        let names: Vec<&str> = filenames.iter().map(std::string::String::as_str).collect();
        let name_filter =
            FilenameFilter::new(&names, filename_match_option.unwrap(), filename_case_sensitiv.unwrap());
        filters.push(Box::new(name_filter));
    }

    if !filecontents.is_empty() {
        let words: Vec<&str> = filecontents
            .iter()
            .map(std::string::String::as_str)
            .collect();
        let file_content_filter =
            FileContentFilter::new(&words, filecontent_match_option.unwrap(), filecontent_case_sensitiv.unwrap());
        filters.push(Box::new(file_content_filter));
    }

    let result_type_filter = EntryTypeFilter::new(results_filter);
    filters.push(Box::new(result_type_filter));

    filters
}

fn display_results(
    results: &[SearchResult],
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
        let entry_action =
            Select::new("What do you want to do?", options.clone()).prompt_skippable()?;

        if let Some(action) = entry_action {
            match action {
                "Open entries" => OpenEntriesDialogue::show(results, max_results),
                "Reveal entries" => RevealEntriesDialogue::show(results, max_results),
                "Show details" => ShowEntriesDialogue::show(results, max_results),
                "Copy entries" => CopyEntriesDialogue::show(results, max_results),
                "Move entries" => MoveEntriesDialogue::show(results, max_results),
                "Delete entries" => DeleteEntriesDialogue::show(results, max_results),
                _ => print_error("Invalid option entered!"),
            }?;
        }

        match Confirm::new("Do you want to do additional things?")
            .with_default(true)
            .prompt()
        {
            Ok(res) => {
                if !res {
                    return Ok(());
                }
            }
            Err(_) => return Ok(()),
        };
    }
}
