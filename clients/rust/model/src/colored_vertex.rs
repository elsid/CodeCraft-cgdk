use serde::Serialize;

use super::*;

#[derive(Clone, Debug, trans::Trans, Serialize)]
pub struct ColoredVertex {
    pub world_pos: Option<Vec2F32>,
    pub screen_offset: Vec2F32,
    pub color: Color,
}
