use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Arguments {
    #[clap(action)]
    pub path: PathBuf,

    #[clap(action, short)]
    pub recursive: bool,

    #[clap(action, long)]
    pub disable_bar: bool,

    #[clap(action, long)]
    pub enable_multi_thread: bool,
}
