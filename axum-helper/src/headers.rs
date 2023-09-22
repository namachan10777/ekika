use std::{fmt::Display, net::IpAddr, str::FromStr};

use http::HeaderValue;

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

pub enum XForwardedProto {
    Http,
    Https,
}

impl AsRef<str> for XForwardedProto {
    fn as_ref(&self) -> &str {
        match self {
            Self::Http => "http",
            Self::Https => "https",
        }
    }
}

impl Header for XForwardedProto {
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
