use {
  crate::{
    arguments::Arguments, fetcher::Fetcher, index::Index, loader::Loader, options::Options,
    server::Server, subcommand::Subcommand,
  },
  anyhow::anyhow,
  axum::{
    extract::{Extension, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
  },
  clap::Parser,
  crates_io_api::{Crate, CratesQueryBuilder, SyncClient},
  elasticsearch::{
    http::transport::Transport,
    indices::{IndicesCreateParts, IndicesExistsParts},
    Elasticsearch, IndexParts, SearchParts, GetParts
  },
  http::Method,
  serde::{Deserialize, Serialize},
  std::{fs, net::SocketAddr, path::PathBuf, process, sync::Arc, time::Duration, time::Instant, thread},
  tower_http::cors::{Any, CorsLayer},
};

mod arguments;
mod fetcher;
mod index;
mod loader;
mod options;
mod server;
mod subcommand;

const USER_AGENT: &str = "rustground";

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

fn main() {
  env_logger::init();

  if let Err(error) = Arguments::parse().run() {
    eprintln!("error: {error}");
    process::exit(1);
  }
}
