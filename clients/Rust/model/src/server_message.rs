use serde::Serialize;

use super::*;

#[derive(Clone, Debug, trans::Trans, Serialize)]
pub enum ServerMessage {
    GetAction {
        player_view: PlayerView,
        debug_available: bool,
    },
    Finish {
    },
    DebugUpdate {
        player_view: PlayerView,
    },
}
