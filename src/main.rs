use {
  crate::{
    arguments::Arguments, elasticsearch_ext::ElasticsearchExt, fetcher::Fetcher, index::Index,
    loader::Loader, options::Options, response_ext::ResponseExt, server::Server,
    subcommand::Subcommand,
  },
  anyhow::anyhow,
  async_trait::async_trait,
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
    http::response::Response,
    http::transport::Transport,
    indices::{IndicesCreateParts, IndicesExistsParts},
    Elasticsearch, GetParts, IndexParts, SearchParts,
  },
  http::Method,
  lazy_static::lazy_static,
  serde::{Deserialize, Serialize},
  serde_json::{json, Value},
  std::{
    fs, marker::Send, net::SocketAddr, path::PathBuf, process, sync::Arc, thread, time::Duration,
  },
  tokio::runtime::Runtime,
  tower_http::cors::{Any, CorsLayer},
};

mod arguments;
mod elasticsearch_ext;
mod fetcher;
mod index;
mod loader;
mod options;
mod response_ext;
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
