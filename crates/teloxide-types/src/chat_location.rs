use serde::{Deserialize, Serialize};

use crate::Location;

/// Represents a location to which a chat is connected.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatLocation {
    /// The location to which the supergroup is connected. Can't be a live
    /// location.
    pub location: Location,
    /// Location address; 1-64 characters, as defined by the chat owner.
    pub address: String,
}
