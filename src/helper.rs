use std::{
    fs::{self, ReadDir},
    path::Path,
    sync::{Arc, Mutex},
    thread,
};

use crate::error::pak;

pub fn read_dir(path: &Path) -> pak::Result<ReadDir> {
    fs::read_dir(path).map_err(|error| pak::Error::from_io_error(error, path))
}

pub fn remove_file(path: &Path) -> pak::Result<()> {
    fs::remove_file(path).map_err(|error| pak::Error::from_io_error(error, path))
}

pub fn remove_dir(path: &Path) -> pak::Result<()> {
    fs::remove_dir(path).map_err(|error| pak::Error::from_io_error(error, path))
}

pub fn count_entries(path: &Path) -> pak::Result<u64> {
    let entries = read_dir(path)?;
    let mut entry_count: u64 = 0;
    let mut thread_handles = Vec::new();
    for entry in entries {
        let entry = entry.unwrap().path();
        if entry.is_dir() && !entry.is_symlink() {
            let new_path = entry;
            let thread_handle = thread::spawn(move || count_entries(&new_path));
            thread_handles.push(thread_handle);
        }
        entry_count += 1;
    }
    for thread_handle in thread_handles {
        entry_count += thread_handle.join().unwrap()?;
    }
    Ok(entry_count)
}

pub fn remove_recursively_multi_thread(
    path: &Path,
    deleted_entries: Arc<Mutex<u64>>,
    thread_limit: Arc<Mutex<u8>>,
) -> pak::Result<()> {
    let entries = read_dir(path)?;
    let mut thread_handles = Vec::new();
    for entry in entries {
        let entry = entry.unwrap().path();
        if entry.is_dir() && !entry.is_symlink() {
            let deleted_entries = deleted_entries.clone();
            let thread_limit = thread_limit.clone();
            if *thread_limit.lock().unwrap() == 0 {
                remove_recursively_multi_thread(&entry, deleted_entries, thread_limit);
            } else {
                *thread_limit.lock().unwrap() -= 1;
                let thread_handle = thread::spawn(move || {
                    remove_recursively_multi_thread(&entry, deleted_entries, thread_limit)
                });
                thread_handles.push(thread_handle);
            }
        } else {
            remove_file(&entry)?;
            *deleted_entries.lock().unwrap() += 1;
        }
    }
    for thread_handle in thread_handles {
        thread_handle.join().unwrap()?;
        *thread_limit.lock().unwrap() += 1;
        *deleted_entries.lock().unwrap() += 1;
    }
    remove_dir(path)?;
    Ok(())
}

pub fn remove_recursively_single_thread(
    path: &Path,
    deleted_entries: Arc<Mutex<u64>>,
) -> pak::Result<()> {
    let entries = read_dir(path)?;
    for entry in entries {
        let entry = entry.unwrap().path();
        if entry.is_file() || entry.is_symlink() {
            remove_file(&entry)?;
        } else {
            let deleted_entries = deleted_entries.clone();
            remove_recursively_single_thread(&entry, deleted_entries)?;
        }
        *deleted_entries.lock().unwrap() += 1;
    }
    remove_dir(path)?;
    Ok(())
}

pub fn remove_single_file(path: &Path, deleted_entries: Arc<Mutex<u64>>) -> pak::Result<()> {
    remove_file(path)?;
    *deleted_entries.lock().unwrap() += 1;
    Ok(())
}
