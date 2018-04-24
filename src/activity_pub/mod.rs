use rocket::http::ContentType;
use rocket::response::Content;
use rocket_contrib::Json;
use serde_json;

pub mod actor;
pub mod webfinger;

pub type Activity = Content<Json>;

pub fn context() -> serde_json::Value {
    json!([
        "https://www.w3.org/ns/activitystreams",
        "https://w3id.org/security/v1",
        {
            "manuallyApprovesFollowers": "as:manuallyApprovesFollowers",
            "sensitive": "as:sensitive",
            "movedTo": "as:movedTo",
            "Hashtag": "as:Hashtag",
            "ostatus":"http://ostatus.org#",
            "atomUri":"ostatus:atomUri",
            "inReplyToAtomUri":"ostatus:inReplyToAtomUri",
            "conversation":"ostatus:conversation",
            "toot":"http://joinmastodon.org/ns#",
            "Emoji":"toot:Emoji",
            "focalPoint": {
                "@container":"@list",
                "@id":"toot:focalPoint"
            },
            "featured":"toot:featured"
        }
    ])
}

pub fn activity(json: serde_json::Value) -> Activity {
    Content(ContentType::new("application", "activity+json"), Json(json))
}
