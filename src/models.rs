use chrono::serde::ts_milliseconds_option;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct AuthorInfo {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub(crate) name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub(crate) website: Option<String>,
    #[serde(skip_serializing, default)]
    pub(crate) email: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Comment {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub uid: Option<u16>,
    pub on: String,
    pub comment: String,
    #[serde(with = "ts_milliseconds_option", default)]
    pub created: Option<DateTime<Utc>>,
    #[serde(
        with = "ts_milliseconds_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub updated: Option<DateTime<Utc>>,
    pub author: AuthorInfo,
}
