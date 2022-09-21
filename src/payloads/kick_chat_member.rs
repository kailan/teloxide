//! Generated by `codegen_payloads`, do not edit by hand.

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::types::{Recipient, True, UserId};

impl_payload! {
    /// Use this method to kick a user from a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the group on their own using invite links, etc., unless [unbanned] first. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns _True_ on success.
    ///
    /// [unbanned]: crate::payloads::UnbanChatMember
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub KickChatMember (KickChatMemberSetters) => True {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: Recipient [into],
            /// Unique identifier of the target user
            pub user_id: UserId,
        }
        optional {
            /// Date when the user will be unbanned, unix time. If user is banned for more than 366 days or less than 30 seconds from the current time they are considered to be banned forever
            #[serde(with = "crate::types::serde_opt_date_from_unix_timestamp")]
            pub until_date: DateTime<Utc> [into],
            /// Pass True to delete all messages from the chat for the user that is being removed. If False, the user will be able to see messages in the group that were sent before the user was removed. Always True for supergroups and channels.
            pub revoke_messages: bool,
        }
    }
}
