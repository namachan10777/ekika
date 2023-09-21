use std::{fmt::Display, str::FromStr};

use super::*;
use crate::{xsd, FunctionalProperty, Remotable, RemotableObjectOrLinkProp, RemotableProperty};
use serde::{Deserialize, Serialize};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-article)
///
/// uri: `https://www.w3.org/ns/activitystreams#Article`
///
/// Represents any kind of multi-paragraph written work.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Article",
///   "name": "What a Crazy Day I Had",
///   "content": "<div>... you will never believe ...</div>",
///   "attributedTo": "http://sally.example.org"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Article {
    #[serde(flatten)]
    pub _super: Object,
}

def_subtypes!(Article, ArticleSubtypes, [Object], { Article });

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-audio)
///
/// uri: `https://www.w3.org/ns/activitystreams#Audio`
///
/// Represents an audio document of any kind.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Audio",
///   "name": "Interview With A Famous Technologist",
///   "url": {
///     "type": "Link",
///     "href": "http://example.org/podcast.mp3",
///     "mediaType": "audio/mp3"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Audio {
    #[serde(flatten)]
    pub _super: Document,
}

def_subtypes!(Audio, AudioSubtypes, [Document, Object], { Audio });

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-document)
///
/// uri: `https://www.w3.org/TR/activitystreams-vocabulary/#dfn-document`
///
/// Represents a document of any kind.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Document",
///   "name": "4Q Sales Forecast",
///   "url": "http://example.org/4q-sales-forecast.pdf"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Document {
    #[serde(flatten)]
    pub _super: Object,
}

def_subtypes!(
    Document,
    DocumentSubtypes,
    [Object],
    {
        Document,
        Audio,
        Image,
        Video,
        Page
    }
);

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-event)
///
/// uri: `https://www.w3.org/ns/activitystreams#Event`
///
/// Represents any kind of event.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Event",
///   "name": "Going-Away Party for Jim",
///   "startTime": "2014-12-31T23:00:00-08:00",
///   "endTime": "2015-01-01T06:00:00-08:00"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Event {
    #[serde(flatten)]
    pub _super: Object,
}

def_subtypes!(Event, EventSubtypes, [Object], { Event });

/// [W3C recommendation]
///
/// uri: `https://www.w3.org/ns/activitystreams#Image`
///
/// An image document of any kind
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Image",
///   "name": "Cat Jumping on Wagon",
///   "url": [
///     {
///       "type": "Link",
///       "href": "http://example.org/image.jpeg",
///       "mediaType": "image/jpeg"
///     },
///     {
///       "type": "Link",
///       "href": "http://example.org/image.png",
///       "mediaType": "image/png"
///     }
///   ]
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Image {
    #[serde(flatten)]
    pub _super: Document,
}

def_subtypes!(Image, ImageSubtypes, [Document, Object], { Image });

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-mention)
///
/// uri: `https://www.w3.org/ns/activitystreams#Mention`
///
/// A specialized [Link] that represents an @mention.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Mention of Joe by Carrie in her note",
///   "type": "Mention",
///   "href": "http://example.org/joe",
///   "name": "Joe"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Mention {
    #[serde(flatten)]
    pub _super: Link,
}

def_subtypes!(Mention, MentionSubtypes, [Link], { Mention });

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-note)
///
/// uri: `https://www.w3.org/ns/activitystreams#Note`
///
/// Represents a short written work typically less than a single paragraph in length.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Note",
///   "name": "A Word of Warning",
///   "content": "Looks like it is going to rain today. Bring an umbrella!"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Note {
    #[serde(flatten)]
    pub _super: Object,
}

def_subtypes!(Note, NoteSubtypes, [Object], { Note });

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-page)
///
/// uri: `https://www.w3.org/ns/activitystreams#Page`
///
/// Represents a Web Page.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Page",
///   "name": "Omaha Weather Report",
///   "url": "http://example.org/weather-in-omaha.html"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Page {
    #[serde(flatten)]
    pub _super: Document,
}

def_subtypes!(Page, PageSubtypes, [Document, Object], { Page });

/// Units for [Place]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Unit {
    /// represents `cm`
    Cm,
    /// represents `ft`
    Feet,
    /// represents `in`
    Inches,
    /// represents `km`
    Km,
    /// represents `m`
    M,
    /// represents `mi`
    Miles,
    /// any units type identified by URL
    Uri(url::Url),
}

impl Default for Unit {
    fn default() -> Self {
        Self::M
    }
}

impl FromStr for Unit {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "cm" => Self::Cm,
            "feet" => Self::Feet,
            "inches" => Self::Inches,
            "km" => Self::Km,
            "m" => Self::M,
            "miles" => Self::Miles,
            other => other.parse().map(Self::Uri)?,
        })
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cm => f.write_str("cm"),
            Self::Feet => f.write_str("feet"),
            Self::Inches => f.write_str("inches"),
            Self::Km => f.write_str("km"),
            Self::M => f.write_str("m"),
            Self::Miles => f.write_str("miles"),
            Self::Uri(uri) => uri.fmt(f),
        }
    }
}

impl<'de> Deserialize<'de> for Unit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <&str as Deserialize>::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl Serialize for Unit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[allow(rustdoc::bare_urls)]
/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-place)
///
/// uri: `https://www.w3.org/ns/activitystreams#Place`
///
/// Represents a logical or physical location. See [5.3 Representing Places](https://www.w3.org/TR/activitystreams-vocabulary/#places) for additional information.
///
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Place",
///   "name": "Fresno Area",
///   "latitude": 36.75,
///   "longitude": 119.7667,
///   "radius": 15,
///   "units": "miles"
/// }
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Place {
    #[serde(flatten)]
    pub _super: Object,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-accuracy)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#accuracy`
    ///
    /// Indicates the accuracy of position coordinates on a [Place] objects.
    /// Expressed in properties of percentage. e.g. "94.0" means "94.0% accurate".
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    pub accuracy: FunctionalProperty<f64>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-altitude)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#altitude`
    ///
    /// Indicates the altitude of a place.
    /// The measurement [Place::units] is indicated using the units property.
    ///
    /// See also [Unit].
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    pub altitude: FunctionalProperty<f64>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-latitude)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#latitude`
    ///
    /// The latitude of a place
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    pub latitude: FunctionalProperty<f64>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-longitude)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#longitude`
    ///
    /// The longitude of a place
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    pub longitude: FunctionalProperty<f64>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-radius)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#radius`
    ///
    /// The radius from the given latitude and longitude for a [Place].
    /// The units is expressed by the [Place::units] property.
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    pub radius: FunctionalProperty<f64>,
    /// [W3C recommendation]()
    ///
    /// uri: `https://www.w3.org/TR/activitystreams-vocabulary/#dfn-units`
    ///
    /// Specifies the measurement units for the [Place::radius] and [Place::altitude] properties on a [Place] object.
    /// If not specified, the default is assumed to be [Unit::M] for "meters".
    #[serde(default = "Unit::default")]
    pub units: Unit,
}

def_subtypes!(Place, PlaceSubtypes, [Object], { Place });

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-profile)
///
/// uri: `https://www.w3.org/ns/activitystreams#Profile`
///
/// A Profile is a content object that describes another Object, typically used to describe
/// [Actor Type](https://www.w3.org/TR/activitystreams-vocabulary/#actor-types) objects.
/// The [Profile::describes] property is used to reference the object being described by the profile.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Profile",
///   "summary": "Sally's Profile",
///   "describes": {
///     "type": "Person",
///     "name": "Sally Smith"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Profile {
    #[serde(flatten)]
    pub _super: Object,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-describes)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#describes`
    ///
    /// On a [Profile] object, the [Profile::describes] property identifies the object described by the Profile.
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    pub describes: FunctionalProperty<Remotable<Object>>,
}

def_subtypes!(Profile, ProfileSubtypes, [Object], { Profile });

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-relationship)
///
/// uri: `https://www.w3.org/ns/activitystreams#Relationship`
///
/// Describes a relationship between two individuals.
/// The [Relationship::subject] and [Relationship::object] properties are used to identify the connected individuals.
/// See [5.2 Representing Relationships Between Entities](https://www.w3.org/TR/activitystreams-vocabulary/#connections) for additional information.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally is an acquaintance of John",
///   "type": "Relationship",
///   "subject": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "relationship": "http://purl.org/vocab/relationship/acquaintanceOf",
///   "object": {
///     "type": "Person",
///     "name": "John"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Relationship {
    #[serde(flatten)]
    pub _super: Object,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-subject)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#subject`
    ///
    /// On a [Relationship] object, the [Relationship::subject] property identifies one of the connected individuals.
    /// For instance, for a [Relationship] object describing "John is related to Sally", [Relationship::subject] would refer to John.
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub subject: RemotableObjectOrLinkProp,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-object)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#object`
    ///
    /// When used within an [crate::Activity], describes the direct object of the activity.
    /// For instance, in the activity "John added a movie to his wishlist", the object of the activity is the movie added.
    /// When used within a [Relationship] describes the entity to which the [Relationship::subject] is related.
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub object: RemotableObjectOrLinkProp,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-relationship)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#relationship`
    ///
    /// On a [Relationship] object,
    /// the [Relationship::relationship] property identifies the kind of relationship that exists between [Relationship::subject] and [Relationship::object].
    #[serde(skip_serializing_if = "RemotableProperty::is_empty")]
    pub relationship: RemotableProperty<Object>,
}

def_subtypes!(Relationship, RelationShipSubtypes, [Object], {
    Relationship
});

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-tombstone)
///
/// uri: `https://www.w3.org/ns/activitystreams#Tombstone`
///
/// A Tombstone represents a content object that has been deleted.
/// It can be used in [crate::Collection]s to signify that there used to be an object at this position, but it has been deleted.
///
/// ```json
/// {
///   "type": "OrderedCollection",
///   "totalItems": 3,
///   "name": "Vacation photos 2016",
///   "orderedItems": [
///     {
///       "type": "Image",
///       "id": "http://image.example/1"
///     },
///     {
///       "type": "Tombstone",
///       "formerType": "Image",
///       "id": "http://image.example/2",
///       "deleted": "2016-03-17T00:00:00Z"
///     },
///     {
///       "type": "Image",
///       "id": "http://image.example/3"
///     }
///   ]
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Tombstone {
    #[serde(flatten)]
    pub _super: Object,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-formertype)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#formerType`
    ///
    /// On a [Tombstone] object, the [Tombstone::former_type] property identifies the type of the object that was deleted.
    #[serde(
        skip_serializing_if = "RemotableProperty::is_empty",
        rename = "formerType"
    )]
    pub former_type: RemotableProperty<Object>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-deleted)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#deleted`
    ///
    /// On a [Tombstone] object, the [Tombstone::deleted] property is a timestamp for when the object was deleted.
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    pub deleted: FunctionalProperty<xsd::DateTime>,
}

def_subtypes!(Tombstone, TombstoneSubtypes, [Object], { Tombstone });

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-video)
///
/// uri: `https://www.w3.org/ns/activitystreams#Video`
///
/// Represents a video document of any kind.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Video",
///   "name": "Puppy Plays With Ball",
///   "url": "http://example.org/video.mkv",
///   "duration": "PT2H"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Video {
    #[serde(flatten)]
    _super: Document,
}

def_subtypes!(Video, VideoSubtypes, [Document, Object], { Video });
