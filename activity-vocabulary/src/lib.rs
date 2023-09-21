use std::marker::PhantomData;

use serde::{de::Visitor, Deserialize, Serialize};
#[macro_use]
pub mod def_util;
pub mod vocab;
pub mod xsd;

/// Not functional property can hav

#[derive(PartialEq, Clone, Debug)]
pub struct Property<T>(Vec<T>);

pub type RemotableOrLinkProp<T> = Property<Or<Remotable<T>, vocab::LinkSubtypes>>;
pub type RemotableObjectOrLinkProp = RemotableOrLinkProp<vocab::ObjectSubtypes>;

struct PropertyVisitor<T> {
    _marker: PhantomData<T>,
}

impl<'de, T: Deserialize<'de>> Visitor<'de> for PropertyVisitor<T> {
    type Value = Property<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("option of array or option")
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Default::default())
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Default::default())
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let content = serde::__private::de::Content::deserialize(deserializer)?;
        let deserializer = serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
        match T::deserialize(deserializer) {
            Ok(just) => Ok(Property(vec![just])),
            Err(just_err) => Vec::<T>::deserialize(deserializer)
                .map_err(|vec_err| serde::de::Error::custom(format!("{vec_err} and {just_err}")))
                .map(Property),
        }
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Property<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_option(PropertyVisitor {
            _marker: Default::default(),
        })
    }
}

impl<T: Serialize> Serialize for Property<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.0.len() == 1 {
            self.0[0].serialize(serializer)
        } else {
            self.0.serialize(serializer)
        }
    }
}

impl<T> Property<T> {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T> Default for Property<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> From<T> for Property<T> {
    fn from(value: T) -> Self {
        Property(vec![value])
    }
}

pub type FunctionalProperty<T> = Option<T>;

#[derive(PartialEq, Clone, Debug)]
pub enum Or<T, U> {
    Prim(T),
    Snd(U),
}

impl<'de, L: Deserialize<'de>, R: Deserialize<'de>> Deserialize<'de> for Or<L, R> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let content = serde::__private::de::Content::deserialize(deserializer)?;
        let deserializer = serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
        match L::deserialize(deserializer) {
            Ok(left) => Ok(Self::Prim(left)),
            Err(left_err) => R::deserialize(deserializer)
                .map_err(|right_err| {
                    serde::de::Error::custom(format!("{left_err} and {right_err}"))
                })
                .map(Self::Snd),
        }
    }
}

impl<T: Serialize, U: Serialize> Serialize for Or<T, U> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Prim(value) => value.serialize(serializer),
            Self::Snd(value) => value.serialize(serializer),
        }
    }
}

//pub type Remotable<T> = Or<T, Link>;
pub type Untypable<T> = Or<T, serde_json::Value>;

impl<L, R> Or<L, R> {
    pub fn prim(&self) -> Option<&L> {
        match self {
            Self::Prim(l) => Some(l),
            Self::Snd(_) => None,
        }
    }
    pub fn snd(&self) -> Option<&R> {
        match self {
            Self::Prim(_) => None,
            Self::Snd(r) => Some(r),
        }
    }
}

impl<P, S> From<P> for Or<P, S> {
    fn from(value: P) -> Self {
        Or::Prim(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Remotable<T> {
    Here(T),
    Remote(url::Url),
}

impl<T: Default> Default for Remotable<T> {
    fn default() -> Self {
        Self::Here(T::default())
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Remotable<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let content = serde::__private::de::Content::deserialize(deserializer)?;
        let deserializer = serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
        match T::deserialize(deserializer) {
            Ok(value) => Ok(Self::Here(value)),
            Err(value_err) => url::Url::deserialize(deserializer)
                .map_err(|url_err| serde::de::Error::custom(format!("{value_err} & {url_err}")))
                .map(Self::Remote),
        }
    }
}

impl<T: Serialize> Serialize for Remotable<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Here(here) => here.serialize(serializer),
            Self::Remote(url) => url.serialize(serializer),
        }
    }
}

impl<T> From<T> for Remotable<T> {
    fn from(value: T) -> Self {
        Self::Here(value)
    }
}

pub type NestedRemotableProperty<T> = Property<Or<Remotable<T>, Vec<Remotable<T>>>>;
pub type RemotableProperty<T> = Property<Remotable<T>>;

#[cfg(any(feature = "resolve-remote-webpki", feature = "resolve-remote-native"))]
impl<T: ToOwned> Remotable<T>
where
    T::Owned: serde::de::DeserializeOwned,
{
    pub async fn resolve(&self) -> Result<std::borrow::Cow<T>, reqwest::Error> {
        match self {
            Self::Here(here) => Ok(std::borrow::Cow::Borrowed(here)),
            Self::Remote(url) => reqwest::Client::new()
                .get(url.clone())
                .header("accept", "application/activity+json")
                .send()
                .await?
                .json::<T::Owned>()
                .await
                .map(std::borrow::Cow::Owned),
        }
    }
}

#[cfg(test)]
mod test {

    use url::Url;

    use super::*;

    #[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
    struct Sample<'a> {
        id: &'a str,
    }

    #[test]
    fn test_linkable_ser_de() {
        let sample = Sample {
            id: "namachan10777",
        };
        let sample_url = "https://activitypub.namachan10777.dev/user/namachan10777";

        let r: Remotable<Sample> = serde_json::from_str(r#"{"id": "namachan10777"}"#).unwrap();
        assert_eq!(r, Remotable::Here(sample));

        let src = format!(r#""{sample_url}""#);
        let r: Remotable<Sample> = serde_json::from_str(&src).unwrap();
        assert_eq!(r, Remotable::Remote(sample_url.parse::<Url>().unwrap()));

        let r: Remotable<&str> = serde_json::from_str(r#""Hello World!""#).unwrap();
        assert_eq!(r, Remotable::Here("Hello World!"));

        assert_eq!(
            &serde_json::to_string(&Remotable::Here(Sample {
                id: "namachan10777"
            }))
            .unwrap(),
            r#"{"id":"namachan10777"}"#,
        );
        assert_eq!(
            serde_json::to_string(&Remotable::<Sample>::Remote(
                sample_url.parse::<Url>().unwrap()
            ))
            .unwrap(),
            format!(r#""{sample_url}""#),
        );
    }
}
