use serde::Serialize;

use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, trans::Trans, Serialize)]
pub struct RepairAction {
    pub target: i32,
}
