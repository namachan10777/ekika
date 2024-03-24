use axum_helper::HttpError;
use std::future::Future;

use crate::model;

pub trait ActivityPub {
    fn get_profile(
        &self,
    ) -> impl Future<Output = Result<model::account::Account, HttpError>> + Send;
}
pub struct ActivityPubServer<S> {
    #[allow(dead_code)]
    service: S,
}

impl<S: ActivityPub> ActivityPubServer<S> {
    pub fn new(service: S) -> Self {
        Self { service }
    }
}
