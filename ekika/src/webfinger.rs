use std::{collections::HashSet, sync::Arc};

use axum::Json;
use axum_helper::{headers::ProxyInfo, HttpError, ToHttpErrorJson};
use maplit::hashset;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::future::Future;
use tracing::debug;

use crate::model::account::Account;

#[derive(Deserialize, Debug)]
pub struct WebfingerQuery {
    resource: url::Url,
}

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(untagged)]
pub enum WebfingerLinks {
    Href {
        rel: String,
        #[serde(rename = "type")]
        mime_type: String,
        href: url::Url,
    },
    Template {
        rel: String,
        template: String,
    },
}

#[derive(Serialize, Deserialize)]
pub struct WebfingerResponse {
    pub subject: url::Url,
    pub aliases: HashSet<url::Url>,
    pub links: HashSet<WebfingerLinks>,
}

pub trait AccountStore {
    type ActorInfo;
    fn query(
        &self,
        name: &str,
    ) -> impl Future<Output = Result<Option<Self::ActorInfo>, HttpError>> + Send;
}

pub async fn webfinger<S>(
    axum::extract::Query(query): axum::extract::Query<WebfingerQuery>,
    axum::extract::State(state): axum::extract::State<Arc<S>>,
    proxy_info: ProxyInfo,
) -> Result<Json<WebfingerResponse>, HttpError>
where
    S: AccountStore<ActorInfo = Account>,
{
    debug!(resource = query.resource.to_string(), "query");
    if query.resource.scheme() != "acct" {
        return Err(HttpError::new_json(
            &json!({"ok": false}),
            http::StatusCode::BAD_REQUEST,
        ));
    }
    let account = query
        .resource
        .path()
        .strip_suffix(&format!("@{}", proxy_info.host))
        .ok_or("not_found")
        .http_error_json(http::StatusCode::NOT_FOUND)?;
    debug!(account = account, "account");
    if state.query(account).await?.is_some() {
        let frontend_profile: url::Url =
            format!("{}://{}/@{account}", proxy_info.proto, proxy_info.host)
                .parse()
                .unwrap();
        let api_endtpoint: url::Url =
            format!("{}://{}/users/{account}", proxy_info.proto, proxy_info.host)
                .parse()
                .unwrap();
        Ok(Json(WebfingerResponse {
            subject: query.resource,
            aliases: maplit::hashset![frontend_profile.clone(), api_endtpoint.clone(),],
            links: hashset! {
                WebfingerLinks::Href {
                    rel: "http://webfinger.net/rel/profile-page".to_string(),
                    mime_type: "text/html".to_string(),
                    href: frontend_profile,
                },
                WebfingerLinks::Href {
                    rel: "self".to_string(),
                    mime_type: "application/activity+json".to_string(),
                    href: api_endtpoint,
                },
            },
        }))
    } else {
        Err(HttpError::new_json(
            &json!({"ok": "false"}),
            http::StatusCode::NOT_FOUND,
        ))
    }
}
