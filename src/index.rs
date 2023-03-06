use super::*;

pub(crate) struct Index {
  client: Elasticsearch,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct SearchPayload {
  pub(crate) time: f64,
  pub(crate) packages: Vec<serde_json::Value>,
}

impl Index {
  const INDEX_ID: &str = "package-index";

  pub(crate) fn open() -> Self {
    Self {
      client: Elasticsearch::default(),
    }
  }

  pub(crate) async fn index(&self, source: PathBuf) -> Result {
    log::info!("Building elasticsearch index...");

    let mapping = serde_json::json!({
      "properties": {
        "id": {"type": "keyword"},
        "name": {"type": "text"},
        "description": {"type": "text"},
        "license": {"type": "keyword"},
        "documentation": {"type": "text"},
        "homepage": {"type": "text"},
        "repository": {"type": "text"},
        "downloads": {"type": "long"},
        "recent_downloads": {"type": "long"},
        "categories": {"type": "keyword"},
        "keywords": {"type": "keyword"},
        "versions": {"type": "long"},
        "max_version": {"type": "keyword"},
        "max_stable_version": {"type": "keyword"},
        "links": {
          "properties": {
            "version_downloads": {"type": "long"},
            "versions": {"type": "keyword"}
          }
        },
        "created_at": {"type": "date"},
        "updated_at": {"type": "date"},
        "exact_match": {"type": "boolean"}
      }
    });

    let response = self
      .client
      .indices()
      .create(IndicesCreateParts::Index(Index::INDEX_ID))
      .body(mapping)
      .send()
      .await?;

    if response.status_code() != StatusCode::CREATED {
      return Err(anyhow!("Failed to create index: {:?}", response));
    }

    log::info!("Index created, writing packages...");

    for package in serde_json::from_str::<Vec<Crate>>(&fs::read_to_string(source)?)? {
      log::info!("Writing package: {}...", package.name);

      let response = self
        .client
        .index(IndexParts::IndexId(Index::INDEX_ID, &package.id))
        .body(serde_json::to_value(&package)?)
        .send()
        .await?;

      if response.status_code() != StatusCode::CREATED {
        return Err(anyhow!("Failed to index document: {:?}", response));
      }
    }

    Ok(())
  }

  pub(crate) async fn search(&self, query: &str) -> Result<SearchPayload> {
    log::info!("Received query: {query}");

    let body = serde_json::json!({
      "query": {
        "query_string": {
          "query": query
        }
      }
    });

    let now = Instant::now();

    let response = self
      .client
      .search(SearchParts::Index(&[Index::INDEX_ID]))
      .body(body)
      .send()
      .await?;

    let elapsed = f64::trunc((now.elapsed().as_secs_f64() * 1000.0) * 100.0) / 100.0;

    if response.status_code() != StatusCode::OK {
      return Err(anyhow!("Failed to execute search: {:?}", response));
    }

    Ok(SearchPayload {
      time: elapsed,
      packages: response.json().await?,
    })
  }
}
