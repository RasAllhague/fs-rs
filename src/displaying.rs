use std::io::stdout;

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};

use crate::{error::FsRsError, search::SearchResult};

pub fn print_error(message: &str) -> Result<(), FsRsError> {
    print_log(message, Color::Red)
}

pub fn print_warning(message: &str) -> Result<(), FsRsError> {
    print_log(message, Color::Yellow)
}

pub fn print_log(message: &str, color: Color) -> Result<(), FsRsError> {
    execute!(
        stdout(),
        SetForegroundColor(color),
        Print(message),
        Print("\n"),
        ResetColor
    )
    .map_err(FsRsError::Crossterm)
}

pub fn print_message(message: &str) -> Result<(), FsRsError> {
    print_log(message, Color::Grey)
}

pub fn print_search_result(search_result: &SearchResult) -> Result<(), FsRsError> {
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
