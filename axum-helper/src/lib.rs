use std::convert::Infallible;

use axum::{
    body::Bytes,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, IntoResponseParts},
};
use http::{HeaderMap, HeaderValue};
use serde::Serialize;

pub mod headers;

pub trait Header {
    fn name() -> &'static str;

    fn decode<'i, I>(values: &mut I) -> Result<Self, axum_extra::headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i axum::http::HeaderValue>;
    fn encode<E: Extend<axum::http::HeaderValue>>(&self, values: &mut E);
}

pub struct TypedHeader<T>(pub T);

enum TypedHeaderRejectionReason {
    Missing,
    Error(axum_extra::headers::Error),
}

pub struct TypedHeaderRejection {
    name: &'static str,
    reason: TypedHeaderRejectionReason,
}

impl std::fmt::Display for TypedHeaderRejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.reason {
            TypedHeaderRejectionReason::Missing => {
                write!(f, "Header of type `{}` was missing", self.name)
            }
            TypedHeaderRejectionReason::Error(err) => {
                write!(f, "{} ({})", err, self.name)
            }
        }
    }
}

impl IntoResponse for TypedHeaderRejection {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}

#[async_trait::async_trait]
impl<S: Send + Sync, T: Header> FromRequestParts<S> for TypedHeader<T> {
    type Rejection = TypedHeaderRejection;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let mut values = parts.headers.get_all(T::name()).iter();
        let is_missing = values.size_hint() == (0, Some(0));
        T::decode(&mut values)
            .map(Self)
            .map_err(|e| TypedHeaderRejection {
                name: T::name(),
                reason: if is_missing {
                    TypedHeaderRejectionReason::Missing
                } else {
                    TypedHeaderRejectionReason::Error(e)
                },
            })
    }
}

#[async_trait::async_trait]
impl<S: Send + Sync, T: Header> FromRequestParts<S> for TypedHeader<Option<T>> {
    type Rejection = TypedHeaderRejection;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let mut values = parts.headers.get_all(T::name()).iter();
        let is_missing = values.size_hint() == (0, Some(0));
        let value = T::decode(&mut values).map_err(|e| TypedHeaderRejection {
            name: T::name(),
            reason: TypedHeaderRejectionReason::Error(e),
        });
        match value {
            Ok(value) => Ok(Self(Some(value))),
            Err(_) if is_missing => Ok(Self(None)),
            Err(e) => Err(e),
        }
    }
}

struct ToValues<'a> {
    state: State<'a>,
}

enum State<'a> {
    First(http::header::Entry<'a, HeaderValue>),
    Latter(http::header::OccupiedEntry<'a, HeaderValue>),
    Tmp,
}

impl<'a> Extend<HeaderValue> for ToValues<'a> {
    fn extend<T: IntoIterator<Item = HeaderValue>>(&mut self, iter: T) {
        for value in iter {
            let entry = match ::std::mem::replace(&mut self.state, State::Tmp) {
                State::First(http::header::Entry::Occupied(mut e)) => {
                    e.insert(value);
                    e
                }
                State::First(http::header::Entry::Vacant(e)) => e.insert_entry(value),
                State::Latter(mut e) => {
                    e.append(value);
                    e
                }
                State::Tmp => unreachable!("ToValues State::Tmp"),
            };
            self.state = State::Latter(entry);
        }
    }
}

impl<T: Header> IntoResponseParts for TypedHeader<T> {
    type Error = Infallible;
    fn into_response_parts(
        self,
        mut res: axum::response::ResponseParts,
    ) -> Result<axum::response::ResponseParts, Self::Error> {
        let entry = res.headers_mut().entry(T::name());
        let mut values = ToValues {
            state: State::First(entry),
        };
        self.0.encode(&mut values);
        Ok(res)
    }
}

impl<T: Header> IntoResponse for TypedHeader<T> {
    fn into_response(self) -> axum::response::Response {
        let mut res = ().into_response();
        let entry = res.headers_mut().entry(T::name());
        let mut values = ToValues {
            state: State::First(entry),
        };
        self.0.encode(&mut values);
        res
    }
}

impl<T: Header> IntoResponseParts for TypedHeader<Option<T>> {
    type Error = Infallible;
    fn into_response_parts(
        self,
        res: axum::response::ResponseParts,
    ) -> Result<axum::response::ResponseParts, Self::Error> {
        if let Some(value) = self.0 {
            TypedHeader(value).into_response_parts(res)
        } else {
            Ok(res)
        }
    }
}

impl<T: Header> IntoResponse for TypedHeader<Option<T>> {
    fn into_response(self) -> axum::response::Response {
        if let Some(value) = self.0 {
            TypedHeader(value).into_response()
        } else {
            ().into_response()
        }
    }
}

pub struct HttpError {
    pub body: Bytes,
    pub mime: mime::Mime,
    pub status: StatusCode,
}

impl HttpError {
    pub fn new_json<M: Serialize>(msg: &M, status: StatusCode) -> Self {
        HttpError {
            body: serde_json::to_vec(msg).unwrap().into(),
            mime: mime::APPLICATION_JSON,
            status,
        }
    }
}

pub trait ToHttpErrorJson {
    type T;
    type E;
    fn http_error_json(self, status: StatusCode) -> Result<Self::T, HttpError>;
}

impl<T, E: Serialize> ToHttpErrorJson for Result<T, E> {
    type T = T;
    type E = E;
    fn http_error_json(self, status: StatusCode) -> Result<Self::T, HttpError> {
        self.map_err(|e| {
            let serialize_err = |_| {
                let e = serde_json::json!({
                    "ok": false,
                    "msg": "falsed to serialize error message",
                });
                serde_json::to_vec(&e).unwrap()
            };
            let body = serde_json::to_vec(&e).unwrap_or_else(serialize_err);
            HttpError {
                body: body.into(),
                mime: mime::APPLICATION_JSON,
                status,
            }
        })
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            HeaderValue::from_str(self.mime.as_ref()).unwrap(),
        );
        (self.status, headers, self.body).into_response()
    }
}

pub struct AccessLogger;
