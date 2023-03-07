use super::*;

pub(crate) struct Fetcher {
  client: SyncClient,
}

impl Fetcher {
  pub(crate) fn new() -> Result<Self> {
    Ok(Self {
      client: SyncClient::new(USER_AGENT, Duration::from_millis(1000))?,
    })
  }

  pub(crate) fn fetch(self, page_size: u64) -> Result {
    let mut crates = Vec::new();

    log::info!("Fetching crates...");

    let mut page = 1;

    loop {
      log::info!("Fetching crates from page {page}...");

      let mut query = CratesQueryBuilder::new().page_size(page_size).build();

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

    fs::write("crates.json", serde_json::to_string(&crates)?)?;

    Ok(())
  }
}
