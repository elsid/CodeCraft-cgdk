use serde::Serialize;

use super::*;

#[derive(Clone, Debug, trans::Trans, Serialize)]
pub struct Action {
    pub entity_actions: std::collections::HashMap<i32, EntityAction>,
}
