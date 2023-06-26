use std::ffi::OsString;

use clap::{Parser, command};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(short, long)]
    pub paths: String,
    #[arg(short, long)]
    pub name_filter: String, 
}