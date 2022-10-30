use crate::domain::clip::field;
use crate::ShortCode;

use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewClip {
    pub title: field::Title,
    pub content: field::Content,
    pub password: field::Password,
    pub expires: field::Expires,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateClip {
    pub title: field::Title,
    pub content: field::Content,
    pub password: field::Password,
    pub expires: field::Expires,
    pub shortcode: field::ShortCode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetClip {
    pub shortcode: ShortCode,
    pub password: field::Password,
}

impl GetClip {
    pub fn from_raw(shortcode: &str) -> Self {
        Self {
            shortcode: ShortCode::from(shortcode),
            password: field::Password::default(),
        }
    }
}

impl From<ShortCode> for GetClip {
    fn from(shortcode: ShortCode) -> Self {
        Self {
            shortcode,
            password: field::Password::default(),
        }
    }
}

impl From<&str> for GetClip {
    fn from(shortcode: &str) -> Self {
        Self::from_raw(shortcode)
    }
}
