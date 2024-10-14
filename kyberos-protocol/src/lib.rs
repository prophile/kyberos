use serde::{Serialize, Deserialize};

/// A struct representing system information.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SystemInfo {
    /// The version of the server.
    pub server_version: String,
}
