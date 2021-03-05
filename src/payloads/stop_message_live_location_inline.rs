// This file is auto generated by `cg` <https://github.com/teloxide/cg> (24572cd + local changes).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::{Message, ReplyMarkup};

impl_payload! {
    /// Use this method to edit live location messages. A location can be edited until its live_period expires or editing is explicitly disabled by a call to [`StopMessageLiveLocation`]. On success, True is returned.
    ///
    /// See also: [`StopMessageLiveLocation`](crate::payloads::StopMessageLiveLocation)
    ///
    /// [`StopMessageLiveLocation`]: crate::payloads::StopMessageLiveLocation
    #[derive(Debug, PartialEq, Clone, Serialize)]
    pub StopMessageLiveLocationInline (StopMessageLiveLocationInlineSetters) => Message {
        required {
            /// Identifier of the inline message
            pub inline_message_id: String [into],
            /// Latitude of new location
            pub latitude: f64,
            /// Longitude of new location
            pub longitude: f64,
        }
        optional {
            /// Additional interface options. A JSON-serialized object for an [inline keyboard], [custom reply keyboard], instructions to remove reply keyboard or to force a reply from the user.
            ///
            /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
            /// [custom reply keyboard]: https://core.telegram.org/bots#keyboards
            pub reply_markup: ReplyMarkup [into],
        }
    }
}
