use std::marker::PhantomData;

use serde::{de::Visitor, Deserialize, Serialize};
pub mod activity;
pub mod actor;
pub mod core;
pub mod object_and_link;
pub mod xsd;

pub use crate::activity::*;
pub use crate::actor::*;
pub use crate::core::*;
pub use crate::object_and_link::*;

/// プロパティは通常複数の値を持ちうる。複数持たないのはFunctionalとマークされたものだけ
#[derive(PartialEq, Debug, Clone)]
pub struct Property<T>(Vec<T>);

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

#[derive(PartialEq, Clone, Debug)]
pub enum Or<T, U> {
    Left(T),
    Right(U),
}

impl<'de, L: Deserialize<'de>, R: Deserialize<'de>> Deserialize<'de> for Or<L, R> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let content = serde::__private::de::Content::deserialize(deserializer)?;
        let deserializer = serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
        match L::deserialize(deserializer) {
            Ok(left) => Ok(Self::Left(left)),
            Err(left_err) => R::deserialize(deserializer)
                .map_err(|right_err| {
                    serde::de::Error::custom(format!("{left_err} and {right_err}"))
                })
                .map(Self::Right),
        }
    }
}

impl<T: Serialize, U: Serialize> Serialize for Or<T, U> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Left(value) => value.serialize(serializer),
            Self::Right(value) => value.serialize(serializer),
        }
    }
}

pub type Linkable<T> = Or<T, Link>;
pub type Untypable<T> = Or<T, serde_json::Value>;

impl<L, R> Or<L, R> {
    pub fn left(&self) -> Option<&L> {
        match self {
            Self::Left(l) => Some(l),
            Self::Right(_) => None,
        }
    }
    pub fn right(&self) -> Option<&R> {
        match self {
            Self::Left(_) => None,
            Self::Right(r) => Some(r),
        }
    }
}

pub type NestedListProperty<T> = Property<Or<Linkable<T>, Vec<Linkable<T>>>>;
pub type LinkableProperty<T> = Property<Linkable<T>>;

#[cfg(test)]
mod test {
    use std::fs;

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

        let r: Linkable<Sample> = serde_json::from_str(r#"{"id": "namachan10777"}"#).unwrap();
        assert_eq!(r, Linkable::Left(sample));

        let src = format!(r#""{sample_url}""#);
        let r: Linkable<Sample> = serde_json::from_str(&src).unwrap();
        assert_eq!(
            r,
            Linkable::Right(sample_url.parse::<Url>().unwrap().into())
        );

        let r: Linkable<&str> = serde_json::from_str(r#""Hello World!""#).unwrap();
        assert_eq!(r, Linkable::Left("Hello World!"));

        assert_eq!(
            &serde_json::to_string(&Linkable::Left(Sample {
                id: "namachan10777"
            }))
            .unwrap(),
            r#"{"id":"namachan10777"}"#,
        );
        assert_eq!(
            serde_json::to_string(&Linkable::<Sample>::Right(
                sample_url.parse::<Url>().unwrap().into()
            ))
            .unwrap(),
            format!(r#""{sample_url}""#),
        );
    }

    #[test]
    fn example4() {
        let s = fs::read_to_string("tests/example4.json").unwrap();
        let x: IntransitiveActivity = serde_json::from_str(&s).unwrap();
        let y = IntransitiveActivity {
            _super: Activity {
                actor: Or::Left(
                    Person {
                        _super: Object {
                            name: "Sally".to_string().into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                    .into(),
                )
                .into(),
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(x, y);
    }
}
