use super::*;

pub(crate) struct Index {
  client: redis::Client,
  crates: Arc<Mutex<BTreeMap<String, Crate>>>,
}

impl Index {
  pub(crate) fn open() -> Result<Self> {
    Ok(Self {
      client: redis::Client::open("redis://localhost:7500")?,
      crates: Arc::new(Mutex::new(BTreeMap::new())),
    })
  }

  pub(crate) fn index(source: PathBuf) -> Result {
    Ok(())
  }
}
