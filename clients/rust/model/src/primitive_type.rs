use serde::Serialize;

use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, trans::Trans, Serialize)]
pub enum PrimitiveType {
    Lines,
    Triangles,
}
