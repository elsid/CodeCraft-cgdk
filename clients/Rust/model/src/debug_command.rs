use serde::Serialize;

use super::*;

#[derive(Clone, Debug, trans::Trans, Serialize)]
pub enum DebugCommand {
    Add {
        data: DebugData,
    },
    Clear {
    },
    SetAutoFlush {
        enable: bool,
    },
    Flush {
    },
}
