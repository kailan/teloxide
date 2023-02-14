use serde::{Deserialize, Serialize};

use crate::User;

/// This object represents a service message about new members invited to a
/// video chat.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct VideoChatParticipantsInvited {
    /// New members that were invited to the video chat
    pub users: Option<Vec<User>>,
}
