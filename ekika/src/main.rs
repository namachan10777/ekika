use std::{net::SocketAddr, sync::Arc};

use aws_sdk_dynamodb::types::AttributeValue;
use axum::{extract, response::IntoResponse, routing};
use axum_helper::{headers::ProxyInfo, HttpError, ToHttpErrorJson};
use clap::Parser;
use ekika::model::account::Account;
use once_cell::sync::Lazy;
use serde::Serialize;
use serde_json::json;
use tower_http::trace::TraceLayer;
use tracing::info;

#[allow(clippy::all, unused)]
mod ap {
    use activity_vocabulary::Unit;
    use activity_vocabulary_core::*;
    include!(concat!(env!("OUT_DIR"), "/vocab.rs"));
}

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
        .http_error_json(http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((
        axum_helper::TypedHeader(axum_helper::headers::ContentType(XRD_XML.clone())),
        txt,
    ))
}

struct State {
    ddb: aws_sdk_dynamodb::Client,
    user_table: String,
}

impl ekika::webfinger::AccountStore for State {
    type ActorInfo = Account;

    async fn query(&self, name: &str) -> Result<Option<Self::ActorInfo>, HttpError> {
        let item = self
            .ddb
            .get_item()
            .table_name(&self.user_table)
            .key("Id", AttributeValue::S(name.to_string()))
            .send()
            .await
            .map_err(|e| format!("{e:?}"))
            .http_error_json(http::StatusCode::INTERNAL_SERVER_ERROR)?;
        let Some(item) = item.item else {
            return Ok(None);
        };
        let user = serde_dynamo::aws_sdk_dynamodb_0_30::from_item(item)
            .map_err(|e| e.to_string())
            .http_error_json(http::StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Some(user))
    }
}

#[derive(Parser)]
struct Opts {
    #[clap(short, long, env)]
    addr: SocketAddr,
    #[clap(short, long, env)]
    json_log: bool,
}

fn init_logger(json: bool) {
    use tracing_subscriber::prelude::*;
    if json {
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
}

async fn ap_get_user(
    extract::Path(user): extract::Path<String>,
    extract::State(state): extract::State<Arc<State>>,
) -> Result<impl IntoResponse, HttpError> {
    let user = state
        .ddb
        .get_item()
        .table_name(state.user_table.clone())
        .key("Id", AttributeValue::S(user))
        .send()
        .await
        .map_err(|e| e.to_string())
        .http_error_json(http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let user = user
        .item
        .ok_or_else(|| json!({"msg": "not found"}))
        .http_error_json(http::StatusCode::NOT_FOUND)?;
    #[allow(unused)]
    let user: Account = serde_dynamo::aws_sdk_dynamodb_0_30::from_item(user)
        .map_err(|e| e.to_string())
        .http_error_json(http::StatusCode::INTERNAL_SERVER_ERROR)?;
    #[allow(unused)]
    let actor = ap::Person::builder();
    Ok("unimplemented")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    init_logger(opts.json_log);

    let sdk_config = aws_config::load_from_env().await;
    let ddb_config = aws_sdk_dynamodb::config::Builder::from(&sdk_config)
        .endpoint_url("http://localhost:8000")
        .build();
    let ddb = aws_sdk_dynamodb::Client::from_conf(ddb_config);

    let state = Arc::new(State {
        ddb,
        user_table: "users".to_string(),
    });

    let router = axum::Router::new()
        .route("/.well-known/host-meta", routing::get(host_meta))
        .route(
            "/.well-known/webfinger",
            routing::get(ekika::webfinger::webfinger),
        )
        .route("/user/:users", routing::get(ap_get_user))
        .with_state(state)
        .layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind(&opts.addr).await?;
    axum::serve(listener, router.into_make_service()).await?;
    Ok(())
}
