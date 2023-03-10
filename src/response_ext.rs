use super::*;

pub(crate) trait ResponseExt {
  fn raise_for_status(self, status: StatusCode) -> Result<Self>
  where
    Self: Sized;
}

impl ResponseExt for Response {
  fn raise_for_status(self, status: StatusCode) -> Result<Self> {
    match self.status_code() == status {
      true => Ok(self),
      _ => Err(anyhow!("Failed to execute request: {:?}", self)),
    }
  }
}
