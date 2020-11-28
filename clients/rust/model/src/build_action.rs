use serde::Serialize;

use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, trans::Trans, Serialize)]
pub struct BuildAction {
    pub entity_type: EntityType,
    pub position: Vec2I32,
}
