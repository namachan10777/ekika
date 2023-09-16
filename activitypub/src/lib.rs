use activity_derive::derive_activity;

derive_activity! {
    types {
        Object {
            uri: "https://www.w3.org/ns/activitystreams#Object",
        },
        Link {
            uri: "https://www.w3.org/ns/activitystreams#Link",
        },
        Activity {
            uri: "https://www.w3.org/ns/activitystreams#Activity",
            extends: Object,
        },
        IntransitiveActivity {
            uri: "https://www.w3.org/ns/activitystreams#IntransitiveActivity",
            extends: Activity,
        },
        Collection {
            uri: "https://www.w3.org/ns/activitystreams#Collection",
            extends: Object,
        },
        OrderedCollection {
            uri: "https://www.w3.org/ns/activitystreams#OrderedCollection",
            extends: Collection,
        },
        CollectionPage {
            uri: "https://www.w3.org/ns/activitystreams#CollectionPage",
            extends: CollectionPage,
        },
        OrderedCollectionPage {
            uri: "https://www.w3.org/ns/activitystreams#OrderedCollectionPage",
            extends: OrderedCollection | CollectionPage,
        },
        Accept {
            uri: "https://www.w3.org/ns/activitystreams#Accept",
            extends: Activity,
        },
        TentativeAccept {
            uri: "https://www.w3.org/ns/activitystreams#TentativeAccept",
            extends: TentativeAccept,
        },
        Add {
            uri: "https://www.w3.org/ns/activitystreams#Add",
            extends: Activity,
        },
        Arrive {
            uri: "https://www.w3.org/ns/activitystreams#Arrive",
            extends: Activity,
        },
        Create {
            uri: "https://www.w3.org/ns/activitystreams#Create",
            extends: Activity,
        },
        Delete {
            uri: "https://www.w3.org/ns/activitystreams#Delete",
            extends: Activity,
        },
        Accept {
            uri: "https://www.w3.org/ns/activitystreams#Accept",
            extends: Activity,
        },
        Follow {
            uri: "https://www.w3.org/ns/activitystreams#Follow",
            extends: Activity,
        },
        Ignore {
            uri: "https://www.w3.org/ns/activitystreams#Ignore",
            extends: Activity,
        },
        Join {
            uri: "https://www.w3.org/ns/activitystreams#Join",
            extends: Activity,
        },
        Leave {
            uri: "https://www.w3.org/ns/activitystreams#Leave",
            extends: Activity,
        },
        Like {
            uri: "https://www.w3.org/ns/activitystreams#Like",
            extends: Activity,
        },
        Offer {
            uri: "https://www.w3.org/ns/activitystreams#Offer",
            extends: Activity,
        },
        Invite {
            uri: "https://www.w3.org/ns/activitystreams#Invite",
            extends: Offer,
        },
        Reject {
            uri: "https://www.w3.org/ns/activitystreams#Reject",
            extends: Activity,
        },
        TentativeReject {
            uri: "https://www.w3.org/ns/activitystreams#TentativeReject",
            extends: Reject,
        },
        Remove {
            uri: "https://www.w3.org/ns/activitystreams#Remove",
            extends: Activity,
        },
        Undo {
            uri: "https://www.w3.org/ns/activitystreams#Undo",
            extends: Activity,
        },
        Update {
            uri: "https://www.w3.org/ns/activitystreams#Update",
            extends: Activity,
        },
        View {
            uri: "https://www.w3.org/ns/activitystreams#View",
            extends: Activity,
        },
        Listen {
            uri: "https://www.w3.org/ns/activitystreams#Listen",
            extends: Activity,
        },
        Read {
            uri: "https://www.w3.org/ns/activitystreams#Read",
            extends: Activity,
        },
        Move {
            uri: "https://www.w3.org/ns/activitystreams#Move",
            extends: Activity,
        },
        Travel {
            uri: "https://www.w3.org/ns/activitystreams#Travel",
            extends: Activity,
        },
        Announce {
            uri: "https://www.w3.org/ns/activitystreams#Announce",
            extends: Activity,
        },
        Block {
            uri: "https://www.w3.org/ns/activitystreams#Block",
            extends: Ignore,
        },
        Flag {
            uri: "https://www.w3.org/ns/activitystreams#Flag",
            extends: Activity,
        },
        Dislike {
            uri: "https://www.w3.org/ns/activitystreams#Dislike",
            extends: Activity,
        },
        Question {
            uri: "https://www.w3.org/ns/activitystreams#Question",
            extends: IntransitiveActivity,
        },
        Application {
            uri: "https://www.w3.org/ns/activitystreams#Application",
            extends: Object,
        },
        Group {
            uri: "https://www.w3.org/ns/activitystreams#Group",
            extends: Object,
        },
        Organization {
            uri: "https://www.w3.org/ns/activitystreams#Organization",
            extends: Object,
        },
        Person {
            uri: "https://www.w3.org/ns/activitystreams#Person",
            extends: Object,
        },
        Service {
            uri: "https://www.w3.org/ns/activitystreams#Service",
            extends: Object,
        },
        Relationship {
            uri: "https://www.w3.org/ns/activitystreams#Relationship",
            extends: Object,
        },
        Article {
            uri: "https://www.w3.org/ns/activitystreams#Article",
            extends: Object,
        },
        Document {
            uri: "https://www.w3.org/ns/activitystreams#Document",
            extends: Object,
        },
        Audio {
            uri: "https://www.w3.org/ns/activitystreams#Audio",
            extends: Document,
        },
        Image {
            uri: "https://www.w3.org/ns/activitystreams#Image",
            extends: Document,
        },
        Video {
            uri: "https://www.w3.org/ns/activitystreams#Video",
            extends: Document,
        },
        Note {
            uri: "https://www.w3.org/ns/activitystreams#Note",
            extends: Activity,
        },
        Page {
            uri: "https://www.w3.org/ns/activitystreams#Page",
            extends: Document,
        },
        Event {
            uri: "https://www.w3.org/ns/activitystreams#Event",
            extends: Object,
        },
        Place {
            uri: "https://www.w3.org/ns/activitystreams#Place",
            extends: Object,
        },
        Mention {
            uri: "https://www.w3.org/ns/activitystreams#Mention",
            extends: Link,
        },
        Profile {
            uri: "https://www.w3.org/ns/activitystreams#Profile",
            extends: Object,
        },
        Tombstone {
            uri: "https://www.w3.org/ns/activitystreams#Tombstone",
            extends: Object,
        },
    },
    properties {
        id {
            uri: "@id",
            domain: Object | Link,
            range: ::url::Url,
            functional: true,
        },
        r#type {
            uri: "@type",
            domain: Object | Link,
            range: ::url::Url,
        },
        actor {
            uri: "https://www.w3.org/ns.activitystreams#actor",
            domain: Activity,
            range: Object | Link,
            subproperty_of: attributedTo
        },
        attachment {
            uri: "https://www.w3.org/ns/activitystreams#attachment",
            domain: Object,
            range: Object | Link,
        },
        attributedTo {
            uri: "https://www.w3.org/ns/activitystreams#attributedTo",
            domain: Link | Object,
            range: Link | Object,
        },
        audience {
            uri: "https://www.w3.org/ns/activitystreams#audience",
            domain: Object,
            range: Object | Link,
        },
        bcc {
           uri: "https://www.w3.org/ns/activitystreams#bcc",
           domain: Object,
           range: Object | Link,
        },
        bto {
            uri: "https://www.w3.org/ns/activitystreams#bto",
            domain: Object,
            range: Object | Link,
        },
        cc {
            uri: "https://www.w3.org/ns/activitystreams#cc",
            domain: Object,
            range: Object | Link,
        },
        context {
            uri: "https://www.w3.org/ns/activitystreams#context",
            domain: Object,
            range: Object | Link,
        },
        current {
            uri: "https://www.w3.org/ns/activitystreams#current",
            domain: Collection,
            range: CollectionPage | Link,
            functional: true
        },
        first {
            uri: "https://www.w3.org/ns/activitystreams#first",
            domain: Collection,
            range: CollectionPage | Link,
            functional: true,
        },
        generator {
            uri: "https://www.w3.org/ns/activitystreams#generator",
            domain: Object,
            range: Object | Link,
        },
        icon {
            uri: "https://www.w3.org/ns/activitystreams#icon",
            domain: Object,
            range: Image | Link,
        },
        image {
            uri: "https://www.w3.org/ns/activitystreams#image",
            domain: Object,
            range: Image | Link,
        },
        inReplyTo {
            uri: "https://www.w3.org/ns/activitystreams#inReplyTo",
            domain: Object,
            range: Object | Link,
        },
        instrument {
            uri: "https://www.w3.org/ns/activitystreams#instrument",
            range: Object | Link,
        },
        last {
            uri: "https://www.w3.org/ns/activitystreams#last",
            domain: Collection,
            range: CollectionPage | Link,
            functional: true,
        },
        location {
            uri: "https://www.w3.org/ns/activitystreams#location",
            domain: Object,
            range: Object | Link,
        },
        items {
            uri: "https://www.w3.org/ns/activitystreams#items",
            domain: Collection,
            range: Object | Link | OrderedList,
        },
        oneOf {
            uri: "https://www.w3.org/ns/activitystreams#oneOf",
            domain: Question,
            range: Object | Link,
        },
        anyOf {
            uri: "https://www.w3.org/ns/activitystreams#anyOf",
            domain: Question,
            range: Object | Link,
        },
        closed {
            uri: "https://www.w3.org/ns/activitystreams#closed",
            domain: Question,
            range: Object | Link | ::chrono::DateTime | bool,
        },
        origin {
            uri: "https://www.w3.org/ns/activitystreams#origin",
            domain: Activity,
            range: Object | Link,
        },
        next {
            uri: "https://www.w3.org/ns/activitystreams#next",
            domain: Activity,
            range: CollectionPage | Link,
            functional: true,
        },
        object {
            uri: "https://www.w3.org/ns/activitystreams#object",
            domain: Activity | Relationship,
            range: Object | Link,
        },
        prev {
            uri: "https://www.w3.org/ns/activitystreams#object",
            domain: CollectionPage,
            range: CollectionPage | Link,
            functional: true,
        },
        preview {
            uri: "https://www.w3.org/ns/activitystreams#preview",
            domain: Link | Object,
            range: Link | Object,
        },
        result {
            uri: "https://www.w3.org/ns/activitystreams#result",
            domain: Activity,
            range: Object | Link
        },
        replies {
            uri: "https://www.w3.org/ns/activitystreams#replies",
            domain: Object,
            range: Collection,
            functional: true,
        },
        tag {
            uri: "https://www.w3.org/ns/activitystreams#tag",
            domain: Object,
            range: Object | Link,
        },
        target {
            uri: "https://www.w3.org/ns/activitystreams#target",
            domain: Activity,
            range: Object | Link,
        },
        to {
            uri: "https://www.w3.org/ns/activitystreams#to",
            domain: Object,
            range: Object | Link,
        },
        url {
            uri: "https://www.w3.org/ns/activitystreams#url",
            domain: Object,
            range: ::url::Url | Link,
        },
        accuracy {
            uri: "https://www.w3.org/ns/activitystreams#accuracy",
            domain: Place,
            range: f64,
            functional: true,
        },
        attitute {
            uri: "https://www.w3.org/ns/activitystreams#altitude",
            domain: Object,
            range: f64,
            functional: true,
        },
        content {
            uri: "https://www.w3.org/ns/activitystreams#content",
            domain: Object,
            range: String,
        },
        name {
            uri: "https://www.w3.org/ns/activitystreams#name",
            domain: Object | Link,
            range: String,
        },
        duration {
            uri: "https://www.w3.org/ns/activitystreams#duration",
            domain: Object,
            range: chrono::Duration,
            functional: true,
        },
        height {
            uri: "https://www.w3.org/ns/activitystreams#height",
            domain: Link,
            range: u64,
            functional: true,
        },
        href {
            uri: "https://www.w3.org/ns/activitystreams#href",
            domain: Link,
            range: ::url::Url,
            functional: true,
        },
        hreflang {
            uri: "https://www.w3.org/ns/activitystreams#hreflang",
            domain: Link,
            range: Lang,
            functional: true,
        },
        partOf {
            uri: "https://www.w3.org/ns/activitystreams#partOf",
            domain: CollectionPage,
            range: Link | Collection,
            functional: true,
        },
        latitude {
            uri: "https://www.w3.org/ns/activitystreams#latitude",
            domain: Place,
            range: f64,
            functional: true,
        },
        longitude {
            uri: "https://www.w3.org/ns/activitystreams#longitude",
            domain: Place,
            range: f64,
            functional: true,
        },
        mediaType {
            uri: "https://www.w3.org/ns/activitystreams#mediaType",
            domain: Link | Object,
            range: ::mime::Mime,
            functional: true,
        },
        endTime {
            uri: "https://www.w3.org/ns/activitystreams#endTime",
            domain: Object,
            range: chrono::DateTime,
            functional: true,
        },
        published {
            uri: "https://www.w3.org/ns/activitystreams#published",
            domain: Object,
            range: chrono::DateTime,
            functional: true,
        },
        startTime {
            uri: "https://www.w3.org/ns/activitystreams#startTime",
            domain: Object,
            range: chrono::DateTime,
            functional: true,
        },
        radius {
            uri: "https://www.w3.org/ns/activitystreams#radius",
            domain: Place,
            range: f64,
            functional: true,
        },
        rel {
            uri: "https://www.w3.org/ns/activitystreams#rel",
            domain: Link,
            range: LinkRelation,
        },
        startIndex {
            uri: "https://www.w3.org/ns/activitystreams#startIndex",
            domain: OrderedCollectionPage,
            range: u64,
            functional: true,
        },
        summary {
            uri: "https://www.w3.org/ns/activitystreams#startIndex",
            domain: Object,
            range: String,
        },
        totalItems {
            uri: "https://www.w3.org/ns/activitystreams#startIndex",
            domain: Collection,
            range: usize,
            functional: true,
        },
        units {
            uri: "https://www.w3.org/ns/activitystreams#units",
            domain: Place,
            range: Units
        },
        updated {
            uri: "https://www.w3.org/ns/activitystreams#updated",
            domain: Object,
            range: chrono::DateTime
        },
        width {
            uri: "https://www.w3.org/ns/activitystreams#width",
            domain: Link,
            range: usize,
            functional: true,
        },
        subject {
            uri: "https://www.w3.org/ns/activitystreams#subject",
            domain: Relationship,
            range: Link | Object,
            functional: true,
        },
        Relationship {
            uri: "https://www.w3.org/ns/activitystreams#Relationship",
            domain: Relationship,
            range: Object,
        },
        describes {
            uri: "https://www.w3.org/ns/activitystreams#describes",
            domain: Profile,
            range: Object,
            functional: true,
        },
        formerType {
            uri: "https://www.w3.org/ns/activitystreams#formerType",
            domain: Tombstone,
            range: Object,
        },
        deleted {
            uri: "https://www.w3.org/ns/activitystreams#deleted",
            domain: Tombstone,
            range: chrono::DateTime,
            functional: true,
        }
    },
}
