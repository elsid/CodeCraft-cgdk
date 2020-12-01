use serde::Serialize;

use super::*;

#[derive(Clone, Debug, PartialEq, Eq, trans::Trans, Serialize)]
pub struct AutoAttack {
    pub pathfind_range: i32,
    pub valid_targets: Vec<EntityType>,
}
