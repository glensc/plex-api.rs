use crate::{MediaContainer, MediaMetadata, MediaType};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct LibraryMediaContainer {
    allow_sync: bool,
    art: Option<String>,
    content: Option<String>,
    identifier: String,
    media_tag_prefix: String,
    media_tag_version: u64,
    title1: Option<String>,
    title2: Option<String>,
    #[serde(rename = "Directory")]
    pub(crate) directory: Option<Vec<DirectoryMediaContainer>>,
    mixed_parents: Option<bool>,
    #[serde(rename = "Metadata")]
    metadata: Option<Vec<MediaMetadata>>,
    #[serde(flatten)]
    media_container: MediaContainer,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
pub struct LibraryMediaContainerOuter {
    #[serde(rename = "MediaContainer")]
    media_container: LibraryMediaContainer,
}

impl From<LibraryMediaContainerOuter> for LibraryMediaContainer {
    fn from(mc: LibraryMediaContainerOuter) -> Self {
        mc.media_container
    }
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
struct DirectoryLocation {
    id: u32,
    path: String,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct DirectoryMediaContainer {
    pub(crate) key: String,
    pub(crate) title: String,
    art: String,
    allow_sync: bool,
    composite: String,
    filters: bool,
    refreshing: bool,
    thumb: String,
    #[serde(rename = "type")]
    media_type: MediaType,
    agent: String,
    scanner: String,
    language: String,
    uuid: Uuid,
    #[serde(with = "chrono::serde::ts_seconds")]
    updated_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    created_at: DateTime<Utc>,
    #[serde(
        default,
        deserialize_with = "crate::serde_helpers::option_seconds_to_datetime"
    )]
    scanned_at: Option<DateTime<Utc>>,
    content: Option<bool>,
    directory: Option<bool>,
    content_changed_at: Option<u64>,
    #[serde(
        default,
        deserialize_with = "crate::serde_helpers::option_bool_from_anything"
    )]
    hidden: Option<bool>,
    #[serde(default, rename = "Location")]
    location: Vec<DirectoryLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_auto_photo_tags: Option<bool>,
}
