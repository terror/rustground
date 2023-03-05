use super::*;

#[derive(Parser)]
pub(crate) struct Options {
  #[clap(long, help = "Path to data source")]
  pub(crate) source: PathBuf,
}
