#![feature(io_error_more)]

use std::sync::{Arc, Mutex};

use clap::Parser;
use error::pak;
use helper::{count_entries, remove_recursively_multi_thread, remove_recursively_single_thread, remove_single_file};
use parser::Arguments;
use progress_bar::start_progress_bar;

mod error;
mod helper;
mod parser;
mod progress_bar;

// remove
fn pak_kon(args: Arguments) -> pak::Result<()> {
    let Arguments {
        path,
        disable_bar,
        enable_multi_thread,
        recursive,
    } = args;
    let entries_count = if recursive { count_entries(&path)? } else { 1 };
    let deleted_entries = Arc::new(Mutex::new(0));

    let progress_bar_handle = if disable_bar {
        None
    } else {
        let progress_bar_handle = start_progress_bar(deleted_entries.clone(), entries_count);
        Some(progress_bar_handle)
    };

    if recursive && enable_multi_thread {
        remove_recursively_single_thread(&path, deleted_entries)?;
    }
    else if recursive {
        remove_recursively_multi_thread(&path, deleted_entries)?;
    }
    else {
        remove_single_file(&path, deleted_entries)?;
    }

    if let Some(progress_bar_handle) = progress_bar_handle {
        progress_bar_handle.join().unwrap();
    };

    Ok(())
}

fn main() {
    let args = Arguments::parse();
    pak_kon(args).unwrap_or_else(|error| println!("{error}"));
}
