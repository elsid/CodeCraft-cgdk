use serde::Serialize;

use super::*;

#[derive(Clone, Debug, trans::Trans, Serialize)]
pub enum ClientMessage {
    DebugMessage {
        command: DebugCommand,
    },
    ActionMessage {
        action: Action,
    },
    DebugUpdateDone {
    },
    RequestDebugState {
    },
}
