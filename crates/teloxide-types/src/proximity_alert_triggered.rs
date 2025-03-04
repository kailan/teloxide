use serde::{Deserialize, Serialize};

use crate::User;

/// This object represents the content of a service message, sent whenever a
/// user in the chat triggers a proximity alert set by another user.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ProximityAlertTriggered {
    /// User that triggered the alert.
    pub traveler: User,

    /// User that set the alert.
    pub watcher: User,

    /// The distance between the users.
    pub distance: u32,
}
