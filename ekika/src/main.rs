#![feature(async_fn_in_trait)]
use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use axum::{response::IntoResponse, routing};
use axum_helper::{headers::ProxyInfo, HttpError, ToHttpErrorJson};
use clap::Parser;
use ekika::model::account::Account;
use once_cell::sync::Lazy;
use reqwest::StatusCode;
use serde::Serialize;
use serde_json::json;
use tokio::sync::RwLock;
use tower_http::trace::TraceLayer;
use tracing::info;

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

async fn host_meta(proxy_info: ProxyInfo) -> Result<impl IntoResponse, HttpError> {
    info!("host-meta");

    let txt = TEMPLATES
        .render(
            "host-meta",
            &HostMetaInput {
                proto: proxy_info.proto.as_ref(),
                host: &proxy_info.host,
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

impl ekika::webfinger::AccountStore for State {
    type ActorInfo = Account;

    async fn query(&self, name: &str) -> Option<Self::ActorInfo> {
        self.accounts.read().await.get(name).cloned()
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
        .route(
            "/.well-known/webfinger",
            routing::get(ekika::webfinger::webfinger),
        )
        .with_state(state)
        .layer(TraceLayer::new_for_http());
    axum::Server::bind(&opts.addr)
        .serve(router.into_make_service())
        .await?;
    Ok(())
}
