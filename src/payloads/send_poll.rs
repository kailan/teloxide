// This file is auto generated by `cg` <https://github.com/teloxide/cg> (24572cd + local changes).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::{ChatId, Message, ParseMode, PollType, ReplyMarkup};

impl_payload! {
    /// Use this method to send phone contacts. On success, the sent [`Message`] is returned.
    ///
    /// [`Message`]: crate::types::Message
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SendPoll (SendPollSetters) => Message {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: ChatId [into],
            /// Poll question, 1-255 characters
            pub question: String [into],
            /// A JSON-serialized list of answer options, 2-10 strings 1-100 characters each
            pub options: Vec<String> [collect],
            /// Poll type, “quiz” or “regular”, defaults to “regular”
            pub type_: PollType,
        }
        optional {
            /// True, if the poll needs to be anonymous, defaults to True
            pub is_anonymous: bool,
            /// True, if the poll allows multiple answers, ignored for polls in quiz mode, defaults to False
            pub allows_multiple_answers: bool,
            /// 0-based identifier of the correct answer option, required for polls in quiz mode
            pub correct_option_id: u8,
            /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters with at most 2 line feeds after entities parsing
            pub explanation: String [into],
            /// Mode for parsing entities in the message text. See [formatting options] for more details.
            ///
            /// [formatting options]: https://core.telegram.org/bots/api#formatting-options
            pub explanation_parse_mode: ParseMode,
            /// Amount of time in seconds the poll will be active after creation, 5-600. Can't be used together with close_date.
            pub open_period: u16,
            /// Point in time (Unix timestamp) when the poll will be automatically closed. Must be at least 5 and no more than 600 seconds in the future. Can't be used together with open_period.
            pub close_date: u64,
            /// Pass True, if the poll needs to be immediately closed. This can be useful for poll preview.
            pub is_closed: bool,
            /// Sends the message [silently]. Users will receive a notification with no sound.
            ///
            /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
            pub disable_notification: bool,
            /// If the message is a reply, ID of the original message
            pub reply_to_message_id: i32,
            /// Additional interface options. A JSON-serialized object for an [inline keyboard], [custom reply keyboard], instructions to remove reply keyboard or to force a reply from the user.
            ///
            /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
            /// [custom reply keyboard]: https://core.telegram.org/bots#keyboards
            pub reply_markup: ReplyMarkup [into],
        }
    }
}
