use {
  crate::{
    arguments::Arguments, fetcher::Fetcher, index::Index, loader::Loader, options::Options,
    server::Server, subcommand::Subcommand,
  },
  anyhow::Error,
  clap::Parser,
  crates_io_api::{Crate, CratesQueryBuilder, SyncClient},
  std::{
    collections::BTreeMap,
    fs,
    path::PathBuf,
    process,
    sync::{Arc, Mutex},
    time::Duration,
  },
};

mod arguments;
mod fetcher;
mod index;
mod loader;
mod options;
mod server;
mod subcommand;

const USER_AGENT: &str = "rustground";

type Result<T = (), E = Error> = std::result::Result<T, E>;

fn main() {
  env_logger::init();

  if let Err(error) = Arguments::parse().run() {
    eprintln!("error: {error}");
    process::exit(1);
  }
}
