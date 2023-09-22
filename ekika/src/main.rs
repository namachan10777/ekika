use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    sync::Arc,
};

use axum::{headers::Host, response::IntoResponse, routing, Json, TypedHeader};
use axum_helper::{
    headers::{XForwardedHost, XForwardedProto},
    HttpError, ToHttpErrorJson,
};
use clap::Parser;
use ekika::model::account::Account;
use maplit::hashset;
use once_cell::sync::Lazy;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::RwLock;
use tower_http::trace::TraceLayer;
use tracing::{debug, info};

static TEMPLATES: Lazy<handlebars::Handlebars> = Lazy::new(|| {
    let mut registry = handlebars::Handlebars::new();
    registry
        .register_template_string("host-meta", include_str!("res/host-meta.xml"))
        .unwrap();
    registry
});

#[derive(Serialize)]
struct HostMetaInput<'a, 'b> {
    proto: &'a str,
    host: &'b str,
}

static XRD_XML: Lazy<mime::Mime> = Lazy::new(|| "application/xrd+xml".parse().unwrap());

async fn host_meta(
    axum_helper::TypedHeader(x_forwarded_host): axum_helper::TypedHeader<Option<XForwardedHost>>,
    axum_helper::TypedHeader(x_forwarded_proto): axum_helper::TypedHeader<Option<XForwardedProto>>,
    TypedHeader(host): TypedHeader<Host>,
) -> Result<impl IntoResponse, HttpError> {
    info!("host-meta");
    let proto = x_forwarded_proto.unwrap_or(XForwardedProto::Http);
    let host = x_forwarded_host
        .map(|host| host.to_string())
        .unwrap_or_else(|| host.to_string());

    let txt = TEMPLATES
        .render(
            "host-meta",
            &HostMetaInput {
                proto: proto.as_ref(),
                host: &host,
            },
        )
        .map_err(|e| json!({"ok": false, "msg": e.to_string()}))
        .http_error_json(StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((
        axum_helper::TypedHeader(axum_helper::headers::ContentType(XRD_XML.clone())),
        txt,
    ))
}

struct State {
    accounts: RwLock<HashMap<String, Account>>,
}

#[derive(Deserialize, Debug)]
struct WebfingerQuery {
    resource: url::Url,
}

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(untagged)]
enum WebfingerLinks {
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
struct WebfingerResponse {
    subject: url::Url,
    aliases: HashSet<url::Url>,
    links: HashSet<WebfingerLinks>,
}

async fn webfinger(
    axum::extract::Query(query): axum::extract::Query<WebfingerQuery>,
    axum_helper::TypedHeader(x_forwarded_host): axum_helper::TypedHeader<Option<XForwardedHost>>,
    axum_helper::TypedHeader(x_forwarded_proto): axum_helper::TypedHeader<Option<XForwardedProto>>,
    axum::extract::State(state): axum::extract::State<Arc<State>>,
    TypedHeader(host): TypedHeader<Host>,
) -> Result<Json<WebfingerResponse>, HttpError> {
    let host = x_forwarded_host
        .map(|host| host.to_string())
        .unwrap_or_else(|| host.to_string());
    let proto = x_forwarded_proto
        .map(|proto| proto.as_ref().to_string())
        .unwrap_or_else(|| "http".to_string());
    debug!(resource = query.resource.to_string(), "query");
    if query.resource.scheme() != "acct" {
        return Err(HttpError::new_json(
            &json!({"ok": false}),
            StatusCode::BAD_REQUEST,
        ));
    }
    let account = query
        .resource
        .path()
        .strip_suffix(&format!("@{host}"))
        .ok_or("not_found")
        .http_error_json(StatusCode::NOT_FOUND)?;
    debug!(account = account, "account");
    if state.accounts.read().await.contains_key(account) {
        let frontend_profile: url::Url = format!("{proto}://{host}/@{account}").parse().unwrap();
        let api_endtpoint: url::Url = format!("{proto}://{host}/users/{account}").parse().unwrap();
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
            StatusCode::NOT_FOUND,
        ))
    }
}

#[derive(Parser)]
struct Opts {
    #[clap(short, long, env)]
    addr: SocketAddr,
    #[clap(short, long, env)]
    json_log: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use tracing_subscriber::prelude::*;

    let opts = Opts::parse();

    if opts.json_log {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "info".into()),
            )
            .with(tracing_subscriber::fmt::layer().json())
            .init();
    } else {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "info".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();
    }

    let acct_namachan10777 = Account {
        kind: ekika::model::account::AccountKind::Person,
        preffered_user_name: "namachan10777".to_owned(),
        name: "namachan10777".to_owned(),
        summary: "namachan10777".to_owned(),
        icon: vec![],
    };

    let state = State {
        accounts: RwLock::new(maplit::hashmap! {
            "admin".to_string() => acct_namachan10777,
        }),
    };
    let state = Arc::new(state);

    let router = axum::Router::new()
        .route("/.well-known/host-meta", routing::get(host_meta))
        .route("/.well-known/webfinger", routing::get(webfinger))
        .with_state(state)
        .layer(TraceLayer::new_for_http());
    axum::Server::bind(&opts.addr)
        .serve(router.into_make_service())
        .await?;
    Ok(())
}
