use serde::{Deserialize, Serialize};

use crate::{Animation, MessageEntity, PhotoSize};

/// This object represents a game.
///
/// Use [@Botfather] to create and edit games, their short names will act as
/// unique identifiers.
///
/// [@Botfather]: https://t.me/botfather
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Game {
    /// Title of the game.
    pub title: String,

    /// Description of the game.
    pub description: String,

    /// Photo that will be displayed in the game message in chats.
    pub photo: Vec<PhotoSize>,

    /// Brief description of the game or high scores included in the game
    /// message. Can be automatically edited to include current high scores
    /// for the game when the bot calls [`SetGameScore`], or manually
    /// edited using [`EditMessageText`]. 0-4096 characters.
    ///
    /// [`SetGameScore`]: crate::payloads::SetGameScore
    /// [`EditMessageText`]: crate::payloads::EditMessageText
    pub text: Option<String>,

    /// Special entities that appear in text, such as usernames, URLs, bot
    /// commands, etc.
    pub text_entities: Option<Vec<MessageEntity>>,

    /// Animation that will be displayed in the game message in chats. Upload
    /// via [@Botfather].
    ///
    /// [@Botfather]: https://t.me/botfather
    pub animation: Option<Animation>,
}
