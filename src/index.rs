use super::*;

pub(crate) struct Index {
  client: Elasticsearch,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct SearchPayload {
  pub(crate) time: f64,
  pub(crate) results: serde_json::Value,
}

impl Index {
  const INDEX_ID: &str = "package-index";

  pub(crate) fn open() -> Result<Self> {
    Ok(Self {
      client: Elasticsearch::new(Transport::single_node("http://localhost:9200")?),
    })
  }

  pub(crate) async fn index(&self, source: PathBuf) -> Result {
    log::info!("Building elasticsearch index...");

    let mapping = serde_json::json!({
      "mappings": {
        "properties": {
          "id": { "type": "keyword" },
          "name": { "type": "text" },
          "description": { "type": "text" },
          "license": { "type": "text" },
          "documentation": { "type": "text" },
          "homepage": { "type": "text" },
          "repository": { "type": "text" },
          "downloads": { "type": "long" },
          "recent_downloads": { "type": "long" },
          "categories": { "type": "text" },
          "keywords": { "type": "text" },
          "versions": { "type": "long" },
          "max_version": { "type": "text" },
          "max_stable_version": { "type": "text" },
          "links": {
            "properties": {
              "owner_team": { "type": "text" },
              "owner_user": { "type": "text" },
              "owners": { "type": "text" },
              "reverse_dependencies": { "type": "text" },
              "version_downloads": { "type": "text" },
              "versions": { "type": "text" }
            }
          },
          "created_at": { "type": "date" },
          "updated_at": { "type": "date" },
          "exact_match": { "type": "boolean" }
        }
      }
    });

    if !self.client.has_index(Index::INDEX_ID).await? {
      let response = self.client.create_index(Index::INDEX_ID, mapping).await?;

      if response.status_code() != StatusCode::OK {
        return Err(anyhow!("Failed to create index: {:?}", response));
      }
    }

    log::info!("Index created, writing packages...");

    for package in serde_json::from_str::<Vec<Crate>>(&fs::read_to_string(source)?)? {
      log::trace!("Checking package: {}...", package.name);

      if self
        .client
        .has_document(Index::INDEX_ID, &package.id)
        .await?
      {
        continue;
      }

      log::trace!("Writing package: {}...", package.name);

      let response = self
        .client
        .index_document(Index::INDEX_ID, &package.id, &package)
        .await?;

      if response.status_code() != StatusCode::CREATED {
        return Err(anyhow!("Failed to index document: {:?}", response));
      }

      thread::sleep(Duration::from_millis(50));
    }

    log::info!("Finished indexing packages.");

    Ok(())
  }

  pub(crate) async fn search(&self, query: &str) -> Result<SearchPayload> {
    log::info!("Received query: {query}");

    let now = Instant::now();

    let response = self
      .client
      .query(
        Index::INDEX_ID,
        serde_json::json!({
          "query": {
            "query_string": {
              "query": query
            }
          }
        }),
      )
      .await?;

    let elapsed = f64::trunc((now.elapsed().as_secs_f64() * 1000.0) * 100.0) / 100.0;

    if response.status_code() != StatusCode::OK {
      return Err(anyhow!("Failed to execute search: {:?}", response));
    }

    Ok(SearchPayload {
      time: elapsed,
      results: response.json().await?,
    })
  }
}
