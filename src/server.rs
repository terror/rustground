use super::*;

#[derive(Parser)]
pub(crate) struct Server {
  #[clap(long, default_value = "8000")]
  port: u16,
}

#[derive(Deserialize)]
pub(crate) struct Params {
  pub(crate) query: String,
}

use tokio::runtime::Runtime;

impl Server {
  pub(crate) fn run(self, source: PathBuf) -> Result {
    Runtime::new()?.block_on(async move {
      log::info!("Initializing index...");

      let index = Arc::new(Index::open()?);

      let clone = index.clone();

      tokio::spawn(async move {
        if let Err(error) = clone.index(source).await {
          log::error!("error: {error}");
        }
      });

      let addr = SocketAddr::from(([127, 0, 0, 1], self.port));

      log::info!("Listening on port: {}...", addr.port());

      axum_server::Server::bind(addr)
        .serve(
          Router::new()
            .route("/search", get(Self::search))
            .layer(Extension(index))
            .layer(
              CorsLayer::new()
                .allow_methods([Method::GET])
                .allow_origin(Any),
            )
            .into_make_service(),
        )
        .await?;

      Ok(())
    })
  }

  async fn search(Query(params): Query<Params>, index: Extension<Arc<Index>>) -> impl IntoResponse {
    match index.search(&params.query).await {
      Ok(payload) => (StatusCode::OK, Json(Some(payload))),
      Err(error) => {
        eprintln!("Error serving request for query {}: {error}", params.query);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
      }
    }
  }
}
