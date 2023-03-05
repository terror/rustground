use super::*;

#[derive(Parser)]
pub(crate) struct Arguments {
  #[clap(flatten)]
  pub(crate) options: Options,
  #[clap(subcommand)]
  subcommand: Subcommand,
}

impl Arguments {
  pub(crate) fn run(self) -> Result {
    match self.subcommand {
      Subcommand::Load(loader) => loader.load(),
      Subcommand::Serve(server) => server.run(self.options.source),
    }
  }
}
