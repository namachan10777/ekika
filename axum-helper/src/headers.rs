use std::{
    borrow::Cow, collections::HashMap, convert::Infallible, fmt::Display, net::IpAddr, str::FromStr,
};

use axum::extract::FromRequestParts;
use http::{request::Parts, HeaderValue};
use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, space0},
    combinator::{eof, map, opt},
    multi::many0,
    sequence::tuple,
    IResult,
};

use crate::Header;

pub struct XForwardedFor(pub Vec<IpAddr>);

impl Header for XForwardedFor {
    fn name() -> &'static str {
        "X-Forwarded-For"
    }

    fn encode<E: Extend<axum::http::HeaderValue>>(&self, values: &mut E) {
        let value = self
            .0
            .iter()
            .map(|ip| ip.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        values.extend(std::iter::once(HeaderValue::from_str(&value).unwrap()))
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, axum::headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i axum::http::HeaderValue>,
    {
        let ips = values.next().ok_or_else(axum::headers::Error::invalid)?;
        let ips = ips.to_str().map_err(|_| axum::headers::Error::invalid())?;
        let ips = ips
            .split(',')
            .map(IpAddr::from_str)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| axum::headers::Error::invalid())?;
        Ok(Self(ips))
    }
}

pub struct XForwardedHost(String);

impl Header for XForwardedHost {
    fn name() -> &'static str {
        "X-Forwarded-Host"
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, axum::headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i axum::http::HeaderValue>,
    {
        let host = values.next().ok_or_else(axum::headers::Error::invalid)?;
        let host = host.to_str().map_err(|_| axum::headers::Error::invalid())?;
        if !host.is_ascii() {
            return Err(axum::headers::Error::invalid());
        }
        Ok(Self(host.to_string()))
    }

    fn encode<E: Extend<axum::http::HeaderValue>>(&self, values: &mut E) {
        values.extend(std::iter::once(HeaderValue::from_str(&self.0).unwrap()));
    }
}

#[derive(Debug)]
pub struct InvalidHostName;

impl Display for InvalidHostName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("InvalidHostName")
    }
}

impl std::error::Error for InvalidHostName {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }

    fn description(&self) -> &str {
        "invalid host name"
    }
}

impl FromStr for XForwardedHost {
    type Err = InvalidHostName;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_ascii() {
            Ok(Self(s.to_string()))
        } else {
            Err(InvalidHostName)
        }
    }
}

impl std::fmt::Display for XForwardedHost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Proto {
    Http,
    Https,
}

impl AsRef<str> for Proto {
    fn as_ref(&self) -> &str {
        match self {
            Self::Http => "http",
            Self::Https => "https",
        }
    }
}

impl Header for Proto {
    fn name() -> &'static str {
        "X-Forwarded-Proto"
    }

    fn encode<E: Extend<axum::http::HeaderValue>>(&self, values: &mut E) {
        values.extend(std::iter::once(
            HeaderValue::from_str(self.as_ref()).unwrap(),
        ))
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, axum::headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i axum::http::HeaderValue>,
    {
        let proto = values.next().ok_or_else(axum::headers::Error::invalid)?;
        let proto = proto
            .to_str()
            .map_err(|_| axum::headers::Error::invalid())?;
        match proto.trim() {
            "http" => Ok(Self::Http),
            "https" => Ok(Self::Https),
            _ => Err(axum::headers::Error::invalid()),
        }
    }
}

pub struct ContentType(pub mime::Mime);

impl Header for ContentType {
    fn name() -> &'static str {
        "Content-Type"
    }

    fn encode<E: Extend<axum::http::HeaderValue>>(&self, values: &mut E) {
        values.extend(std::iter::once(
            HeaderValue::from_str(self.0.as_ref()).unwrap(),
        ));
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, axum::headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i axum::http::HeaderValue>,
    {
        let mime = values.next().ok_or_else(axum::headers::Error::invalid)?;
        let mime = mime.to_str().map_err(|_| axum::headers::Error::invalid())?;
        let mime = mime.parse().map_err(|_| axum::headers::Error::invalid())?;
        Ok(Self(mime))
    }
}

pub struct ProxyInfo {
    pub host_proxied: bool,
    pub proto_proxied: bool,
    pub host: String,
    pub proto: String,
}

struct Forwarded<'a> {
    proto: Option<&'a str>,
    host: Option<&'a str>,
}

fn parse_forwared(src: &str) -> IResult<&str, Forwarded> {
    let (src, _) = space0(src)?;
    let pair = map(
        tuple((
            alpha1,
            tag("="),
            take_while1(|c: char| c.is_alphanumeric() && c == '.'),
            opt(tag(";")),
        )),
        |(key, _, value, _)| (key, value),
    );
    let (src, pairs) = many0(pair)(src)?;
    let (src, _) = space0(src)?;
    let (src, _) = eof(src)?;
    let pairs = pairs.into_iter().collect::<HashMap<_, _>>();
    Ok((
        src,
        Forwarded {
            proto: pairs.get("proto").map(|v| *v),
            host: pairs.get("host").map(|v| *v),
        },
    ))
}

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for ProxyInfo {
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let (forwarded_host, forwarded_proto) =
            if let Some(forwared) = parts.headers.get("Forwarded") {
                let forwared = String::from_utf8_lossy(forwared.as_bytes());
                if let Ok((_, forwared)) = parse_forwared(&forwared) {
                    (
                        forwared.host.map(ToString::to_string),
                        forwared.proto.map(ToString::to_string),
                    )
                } else {
                    (None, None)
                }
            } else {
                (None, None)
            };
        let forwarded_host = forwarded_host.map(Cow::Owned);
        let forwarded_proto = forwarded_proto.map(Cow::Owned);

        let x_forwarded_host = parts
            .headers
            .get("XForwardedHost")
            .map(|value| String::from_utf8_lossy(value.as_bytes()));

        let x_forwarded_proto = parts
            .headers
            .get("XForwardedProto")
            .map(|value| String::from_utf8_lossy(value.as_bytes()));

        let host = String::from_utf8_lossy(parts.headers.get("Host").unwrap().as_bytes());

        let proto = Cow::Borrowed("http");

        let proto_proxied = forwarded_proto.is_some() || x_forwarded_proto.is_some();
        let host_proxied = forwarded_host.is_some() || x_forwarded_host.is_some();

        let host = forwarded_host
            .or(x_forwarded_host)
            .unwrap_or(host)
            .to_string();

        let proto = forwarded_proto
            .or(x_forwarded_proto)
            .unwrap_or(proto)
            .to_string();

        Ok(ProxyInfo {
            proto_proxied,
            host_proxied,
            host,
            proto,
        })
    }
}
