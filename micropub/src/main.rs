use std::net::SocketAddr;

use axum::{
    headers::{ContentType, Host},
    response::IntoResponse,
    routing, TypedHeader,
};
use axum_helper::{
    headers::{XForwardedHost, XForwardedProto},
    HttpError, ToHttpErrorJson,
};
use clap::Parser;
use once_cell::sync::Lazy;
use reqwest::StatusCode;
use serde::Serialize;
use serde_json::json;
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
    Ok((axum::TypedHeader(ContentType::xml()), txt))
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

    let router = axum::Router::new()
        .route("/.well-known/host-meta", routing::get(host_meta))
        .layer(TraceLayer::new_for_http());
    axum::Server::bind(&opts.addr)
        .serve(router.into_make_service())
        .await?;
    Ok(())
}
