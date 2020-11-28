use serde::Serialize;

use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, trans::Trans, Serialize)]
pub struct Vec2I32 {
    pub x: i32,
    pub y: i32,
}
