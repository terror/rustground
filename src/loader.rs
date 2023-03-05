use super::*;

#[derive(Parser)]
pub(crate) struct Loader {
  #[clap(long, default_value = "50", help = "Crate page size")]
  page_size: u64,
}

impl Loader {
  pub(crate) fn load(self) -> Result {
    Fetcher::new()?.fetch(self.page_size)
  }
}
