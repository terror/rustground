use super::*;

#[derive(Parser)]
pub(crate) struct Server {
  #[clap(long, default_value = "8000")]
  port: u16,
}

impl Server {
  pub(crate) fn run(self, source: PathBuf) -> Result {
    Ok(())
  }
}
