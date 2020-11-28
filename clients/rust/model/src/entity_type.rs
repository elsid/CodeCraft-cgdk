use serde::Serialize;

use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, trans::Trans, Serialize)]
pub enum EntityType {
    Wall,
    House,
    BuilderBase,
    BuilderUnit,
    MeleeBase,
    MeleeUnit,
    RangedBase,
    RangedUnit,
    Resource,
    Turret,
}
