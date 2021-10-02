use crate::common::*;

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackFile {
    pub id: SlackFileId,
    pub created: Option<SlackDateTime>,
    pub timestamp: Option<SlackDateTime>,
    pub name: Option<String>,
    pub title: Option<String>,
    pub mimetype: Option<String>,
    pub filetype: Option<String>,
    pub pretty_type: Option<String>,
    pub user: Option<SlackUserId>,
    pub editable: Option<bool>,
    pub size: Option<u32>,
    pub mode: Option<String>,
    pub is_external: Option<bool>,
    pub external_type: Option<String>,
    pub is_public: Option<bool>,
    pub public_url_shared: Option<bool>,
    pub display_as_bot: Option<bool>,
    pub username: Option<String>,
    pub url_private: Option<String>,
    pub url_private_download: Option<String>,
    pub thumb_64: Option<String>,
    pub thumb_80: Option<String>,
    pub thumb_360: Option<String>,
    pub thumb_360_w: Option<u32>,
    pub thumb_360_h: Option<u32>,
    pub thumb_160: Option<String>,
    pub thumb_360_gif: Option<String>,
    pub image_exif_rotation: Option<u8>,
    pub original_w: Option<u32>,
    pub original_h: Option<u32>,
    pub deanimate_gif: Option<String>,
    pub pjpeg: Option<String>,
    pub permalink: Option<String>,
    pub permalink_public: Option<String>,
    pub comments_count: Option<u32>,
    pub channels: Option<Vec<SlackChannelId>>,
    pub groups: Option<Vec<SlackChannelId>>,
    pub ims: Option<Vec<SlackChannelId>>,
}
