use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum Action {
    Tick,
    Next,
    Previous,
    Enter,
    Render,
    Quit,
    Error(String),
    Help,
}
