use {
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

const USER_AGENT: &str = "rustground";

#[derive(Parser)]
struct Options {
  #[clap(long, default_value = "50", help = "Loader page size")]
  page_size: u64,
  #[clap(long, help = "Path to data source")]
  source: PathBuf,
}

#[derive(Parser)]
struct Arguments {
  #[clap(flatten)]
  options: Options,
  #[clap(subcommand)]
  subcommand: Subcommand,
}

impl Arguments {
  fn run(self) -> Result {
    match self.subcommand {
      Subcommand::Load => Loader::new(self.options)?.load(),
      Subcommand::Serve => Server::new(self.options.source).run(),
    }
  }
}

struct Server {
  source: PathBuf,
}

impl Server {
  fn new(source: PathBuf) -> Self {
    Self { source }
  }

  fn run(self) -> Result {
    Ok(())
  }
}

#[derive(Parser)]
enum Subcommand {
  Load,
  Serve,
}

struct Loader {
  client: SyncClient,
  options: Options,
}

impl Loader {
  fn new(options: Options) -> Result<Self> {
    Ok(Self {
      client: SyncClient::new(USER_AGENT, Duration::from_millis(1000))?,
      options,
    })
  }

  fn load(self) -> Result {
    let mut crates = Vec::new();

    log::info!("Fetching crates...");

    let mut page = 1;

    loop {
      log::info!("Fetching crates from page {page}...");

      let mut query = CratesQueryBuilder::new()
        .page_size(self.options.page_size.into())
        .build();

      query.set_page(page);

      let response = self.client.crates(query)?;

      if response.crates.is_empty() {
        break;
      }

      log::trace!(
        "Fetched crates: {:?}",
        response
          .crates
          .iter()
          .map(|c| c.name.clone())
          .collect::<Vec<String>>()
      );

      crates.extend(response.crates);

      page += 1;
    }

    log::info!("Writing crates to data source...");

    fs::write(self.options.source, serde_json::to_string(&crates)?)?;

    Ok(())
  }
}

struct Index {
  client: redis::Client,
  crates: Arc<Mutex<BTreeMap<String, Crate>>>,
}

impl Index {
  fn open(url: &str) -> Result<Self> {
    Ok(Self {
      client: redis::Client::open(url)?,
      crates: Arc::new(Mutex::new(BTreeMap::new())),
    })
  }
}

type Result<T = (), E = Error> = std::result::Result<T, E>;

fn main() {
  env_logger::init();

  if let Err(error) = Arguments::parse().run() {
    eprintln!("error: {error}");
    process::exit(1);
  }
}
