use serde::Deserialize; // library for serializing and deserializing objects

/// Represents metadata about a user from the Github API. Most non-primitives
/// are wrapped in an `Option` to allow for the values to be missing, as not
/// all values are always present for all users.
// This is what a struct declaration looks like. The #[derive] part implements
// certain essential methods for deserializing from JSON for us
#[derive(Deserialize)]
pub struct User { //pub is required for exporting
    pub login: Option<String>, //pub is required for accessing with dot syntax
    pub id: i32,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    // pub type: Option<String>, // cant use this field as type is a keyword
    pub site_admin: bool,
    pub name: Option<String>,
    pub company: Option<String>,
    pub blog: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub hireable: Option<String>,
    pub bio: Option<String>,
    pub twitter_username: Option<String>,
    pub public_repos: i32,
    pub public_gists: i32,
    pub followers: i32,
    pub following: i32,
    pub created_at: Option<String>,
    pub updated_at: Option<String>
}
