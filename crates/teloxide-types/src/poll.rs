use crate::{MessageEntity, PollType};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// This object contains information about a poll.
///
/// [The official docs](https://core.telegram.org/bots/api#poll).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Poll {
    /// Unique poll identifier.
    pub id: String,

    /// Poll question, 1-255 characters.
    pub question: String,

    /// List of poll options.
    pub options: Vec<PollOption>,

    /// `true`, if the poll is closed.
    pub is_closed: bool,

    /// Total number of users that voted in the poll
    pub total_voter_count: i32,

    /// True, if the poll is anonymous
    pub is_anonymous: bool,

    /// Poll type, currently can be “regular” or “quiz”
    #[serde(rename = "type")]
    pub poll_type: PollType,

    /// True, if the poll allows multiple answers
    pub allows_multiple_answers: bool,

    /// 0-based identifier of the correct answer option. Available only for
    /// polls in the quiz mode, which are closed, or was sent (not
    /// forwarded) by the bot or to the private chat with the bot.
    pub correct_option_id: Option<u8>,

    /// Text that is shown when a user chooses an incorrect answer or taps on
    /// the lamp icon in a quiz-style poll, 0-200 characters.
    pub explanation: Option<String>,

    /// Special entities like usernames, URLs, bot commands, etc. that appear in
    /// the explanation.
    pub explanation_entities: Option<Vec<MessageEntity>>,

    /// Amount of time in seconds the poll will be active after creation.
    pub open_period: Option<u16>,

    /// Point in time when the poll will be automatically closed.
    #[serde(default, with = "crate::serde_opt_date_from_unix_timestamp")]
    pub close_date: Option<DateTime<Utc>>,
}

/// This object contains information about one answer option in a poll.
///
/// [The official docs](https://core.telegram.org/bots/api#polloption).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct PollOption {
    /// Option text, 1-100 characters.
    pub text: String,

    /// Number of users that voted for this option.
    pub voter_count: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data = r#"
        {
            "allows_multiple_answers": false,
            "id": "5377643193141559299",
            "is_anonymous": true,
            "is_closed": false,
            "options": [
                {
                    "text": "1",
                    "voter_count": 1
                },
                {
                    "text": "2",
                    "voter_count": 0
                },
                {
                    "text": "3",
                    "voter_count": 0
                },
                {
                    "text": "4",
                    "voter_count": 0
                },
                {
                    "text": "5",
                    "voter_count": 0
                }
            ],
            "question": "Rate me from 1 to 5.",
            "total_voter_count": 1,
            "type": "regular"
        }
        "#;
        serde_json::from_str::<Poll>(data).unwrap();
    }
}
