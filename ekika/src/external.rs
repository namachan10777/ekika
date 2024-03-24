use axum_helper::HttpError;

use crate::model;

pub trait ActivityPub {
    async fn get_profile(&self) -> Result<model::account::Account, HttpError>;
}
pub struct ActivityPubServer<S> {
    service: S,
}

impl<S: ActivityPub> ActivityPubServer<S> {
    pub fn new(service: S) -> Self {
        Self { service }
    }
}
