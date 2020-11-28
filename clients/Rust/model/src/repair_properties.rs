use serde::Serialize;

use super::*;

#[derive(Clone, Debug, trans::Trans, Serialize)]
pub struct RepairProperties {
    pub valid_targets: Vec<EntityType>,
    pub power: i32,
}
