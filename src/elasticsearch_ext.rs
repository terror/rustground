use super::*;

#[async_trait]
pub(crate) trait ElasticSearchExt {
  async fn create_index<T: Serialize + Send>(&self, index_id: &str, body: T) -> Result<Response>;
  async fn has_document(&self, index_id: &str, document_id: &str) -> Result<bool>;
  async fn has_index(&self, index_id: &str) -> Result<bool>;
  async fn index_document<T: Serialize + Send>(
    &self,
    index_id: &str,
    document_id: &str,
    document: T,
  ) -> Result<Response>;
  async fn query<T: Serialize + Send>(&self, index_id: &str, body: T) -> Result<Response>;
}

#[async_trait]
impl ElasticSearchExt for Elasticsearch {
  async fn create_index<T: Serialize + Send>(&self, index_id: &str, body: T) -> Result<Response> {
    Ok(
      self
        .indices()
        .create(IndicesCreateParts::Index(index_id))
        .body(body)
        .send()
        .await?,
    )
  }

  async fn has_document(&self, index_id: &str, document_id: &str) -> Result<bool> {
    Ok(
      self
        .get(GetParts::IndexId(index_id, document_id))
        .send()
        .await?
        .status_code()
        .is_success(),
    )
  }

  async fn has_index(&self, index_id: &str) -> Result<bool> {
    Ok(
      self
        .indices()
        .exists(IndicesExistsParts::Index(&[index_id]))
        .send()
        .await?
        .status_code()
        .is_success(),
    )
  }

  async fn index_document<T: Serialize + Send>(
    &self,
    index_id: &str,
    document_id: &str,
    body: T,
  ) -> Result<Response> {
    Ok(
      self
        .index(IndexParts::IndexId(index_id, document_id))
        .body(body)
        .send()
        .await?,
    )
  }

  async fn query<T: Serialize + Send>(&self, index_id: &str, body: T) -> Result<Response> {
    Ok(
      self
        .search(SearchParts::Index(&[index_id]))
        .body(body)
        .send()
        .await?,
    )
  }
}
