use serde::Serialize;

use super::*;

#[derive(Clone, Debug, trans::Trans, Serialize)]
pub struct Vec2F32 {
    pub x: f32,
    pub y: f32,
}
