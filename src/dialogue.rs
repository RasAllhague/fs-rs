use std::fs::{self, copy, remove_file};

use inquire::{Confirm, MultiSelect, Text};

use crate::{
    displaying::{print_error, print_message, print_search_result, print_warning},
    error::FsRsError,
    search::SearchResult,
};

pub struct CopyEntriesDialogue;

pub struct ShowEntriesDialogue;

pub struct OpenEntriesDialogue;

pub struct RevealEntriesDialogue;

pub struct MoveEntriesDialogue;

pub struct DeleteEntriesDialogue;

impl CopyEntriesDialogue {
    pub fn show(results: &[SearchResult], max_results: usize) -> Result<(), FsRsError> {
        let selected = MultiSelect::new("Which entries do you want to copy?", results.to_vec())
            .with_page_size(max_results)
            .prompt_skippable()?;

        if let Some(entries) = selected {
            for entry in entries.iter() {
                Self::copy_entry(entry)?;
            }
        }

        Ok(())
    }

    fn copy_entry(entry: &SearchResult) -> Result<(), FsRsError> {
        print_warning(&format!("Moving entry: {:?}.", entry.path()))?;
        let move_to = Text::new("Where do you want to move it too?").prompt_skippable()?;

        if let Some(file_name) = move_to {
            match entry {
                SearchResult::Directory {
                    path,
                    name: _,
                    metadata: _,
                } => todo!(),
                SearchResult::File {
                    path,
                    name: _,
                    metadata: _,
                } => {
                    if let Err(why) = copy(path, file_name) {
                        print_error(&format!("Could not copy file: {}", why))?;
                    }
                }
                SearchResult::SymLink {
                    path,
                    name: _,
                    metadata: _,
                } => todo!(),
            }

            print_message("Done!")?;
        }

        Ok(())
    }
}

impl ShowEntriesDialogue {
    pub fn show(results: &[SearchResult], max_results: usize) -> Result<(), FsRsError> {
        let selected = MultiSelect::new(
            "Which entries do you want to see details from?",
            results.to_vec(),
        )
        .with_page_size(max_results)
        .prompt_skippable()?;

        if let Some(entries) = selected {
            for entry in entries.iter() {
                print_search_result(entry)?;
            }
        }

        Ok(())
    }
}

impl OpenEntriesDialogue {
    pub fn show(results: &[SearchResult], max_results: usize) -> Result<(), FsRsError> {
        let selected = MultiSelect::new("Which entries do you want to open?", results.to_vec())
            .with_page_size(max_results)
            .prompt_skippable()?;

        if let Some(entries) = selected {
            for entry in entries.iter() {
                print_warning(&format!("Opening: {:?}!", entry.path()))?;

                if let Err(why) = opener::open(entry.path()) {
                    print_error(&format!(
                        "Failed to open entry with default program, why {why}."
                    ))?;
                } else {
                    print_message("Opened entry.")?;
                }
            }
        }

        Ok(())
    }
}

impl RevealEntriesDialogue {
    pub fn show(results: &[SearchResult], max_results: usize) -> Result<(), FsRsError> {
        let selected = MultiSelect::new("Which entries do you want to open?", results.to_vec())
            .with_page_size(max_results)
            .prompt_skippable()?;

        if let Some(entries) = selected {
            for entry in entries.iter() {
                print_warning(&format!("Opening: {:?}!", entry.path()))?;

                if let Err(why) = opener::reveal(entry.path()) {
                    print_error(&format!(
                        "Failed to open entry with default program, why {why}."
                    ))?;
                } else {
                    print_message("Opened entry.")?;
                }
            }
        }

        Ok(())
    }
}

impl MoveEntriesDialogue {
    pub fn show(results: &[SearchResult], max_results: usize) -> Result<(), FsRsError> {
        let selected = MultiSelect::new("Which entries do you want to move?", results.to_vec())
            .with_page_size(max_results)
            .prompt_skippable()?;

        if let Some(entries) = selected {
            for entry in entries.iter() {
                Self::move_entry(entry)?;
            }
        }

        Ok(())
    }

    fn move_entry(entry: &SearchResult) -> Result<(), FsRsError> {
        print_warning(&format!("Moving entry: {:?}.", entry.path()))?;
        let move_to = Text::new("Where do you want to move it too?").prompt_skippable()?;

        if let Some(file_name) = move_to {
            match entry {
                SearchResult::Directory {
                    path,
                    name: _,
                    metadata: _,
                } => todo!(),
                SearchResult::File {
                    path,
                    name: _,
                    metadata: _,
                } => match copy(path, file_name) {
                    Ok(_) => remove_file(path)?,
                    Err(why) => print_error(&format!("Could not move file: {}", why))?,
                },
                SearchResult::SymLink {
                    path,
                    name: _,
                    metadata: _,
                } => todo!(),
            }

            print_message("Done!")?;
        }

        Ok(())
    }
}

impl DeleteEntriesDialogue {
    pub fn show(results: &[SearchResult], max_results: usize) -> Result<(), FsRsError> {
        let selected =
            MultiSelect::new("Which entries do you want to see delete?", results.to_vec())
                .with_page_size(max_results)
                .prompt_skippable()?;

        if let Some(entries) = selected {
            for entry in entries.iter() {
                Self::delete_entry(entry)?;
            }
        }

        Ok(())
    }

    fn delete_entry(entry: &SearchResult) -> Result<(), FsRsError> {
        print_warning(&format!("Attempting to delete: {:?}!", entry.path()))?;
        let confirmation = Confirm::new("Are you sure?").with_default(false).prompt()?;

        if confirmation {
            match entry {
                SearchResult::Directory {
                    path,
                    name: _,
                    metadata: _,
                } => fs::remove_dir_all(path),
                SearchResult::File {
                    path,
                    name: _,
                    metadata: _,
                } => fs::remove_file(path),
                SearchResult::SymLink {
                    path,
                    name: _,
                    metadata: _,
                } => fs::remove_file(path),
            }?;
        }

        Ok(())
    }
}
