use super::*;

lazy_static! {
  static ref SCHEMA: Value = json!({
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
}

pub(crate) struct Index {
  client: Elasticsearch,
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

    if !self.client.has_index(Index::INDEX_ID).await? {
      self
        .client
        .create_index(Index::INDEX_ID, SCHEMA.to_owned())
        .await?
        .raise_for_status(StatusCode::OK)?;
    }

    log::info!("Index created, writing packages...");

    for package in serde_json::from_str::<Vec<Crate>>(&fs::read_to_string(source)?)? {
      log::trace!("Checking package {}...", package.name);

      if self
        .client
        .has_document(Index::INDEX_ID, &package.id)
        .await?
      {
        continue;
      }

      log::trace!("Indexing package {}...", package.name);

      self
        .client
        .index_document(Index::INDEX_ID, &package.id, &package)
        .await?
        .raise_for_status(StatusCode::CREATED)?;

      thread::sleep(Duration::from_millis(50));
    }

    log::info!("Finished indexing packages.");

    Ok(())
  }

  pub(crate) async fn search(&self, query: &str) -> Result<serde_json::Value> {
    log::info!("Received query: {query}");

    let response = self
      .client
      .query(
        Index::INDEX_ID,
        json!({
          "query": {
            "query_string": {
              "query": query
            }
          }
        }),
      )
      .await?
      .raise_for_status(StatusCode::OK)?;

    Ok(response.json().await?)
  }
}
