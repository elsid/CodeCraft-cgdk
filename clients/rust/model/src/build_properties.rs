use serde::Serialize;

use super::*;

#[derive(Clone, Debug, trans::Trans, Serialize)]
pub struct BuildProperties {
    pub options: Vec<EntityType>,
    pub init_health: Option<i32>,
}
