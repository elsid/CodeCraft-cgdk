use serde::Serialize;

use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, trans::Trans, Serialize)]
pub struct Player {
    pub id: i32,
    pub score: i32,
    pub resource: i32,
}
