use serde::{Deserialize, Serialize};

use crate::WebAppInfo;

/// This object describes the bot's menu button in a private chat.
///
/// If a menu button other than `MenuButton::Default` is set for a private chat,
/// then it is applied in the chat. Otherwise the default menu button is
/// applied. By default, the menu button opens the list of bot commands.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum MenuButton {
    /// Represents a menu button, which opens the bot's list of commands.
    Commands,

    /// Represents a menu button, which launches a [Web App].
    ///
    /// [Web App]: https://core.telegram.org/bots/webapps
    WebApp {
        /// Text on the button.
        text: String,

        /// Description of the Web App that will be launched when the user
        /// presses the button. The Web App will be able to send an arbitrary
        /// message on behalf of the user using the method
        /// [`AnswerWebAppQuery`].
        ///
        /// [`AnswerWebAppQuery`]: crate::payloads::AnswerWebAppQuery
        web_app: WebAppInfo,
    },

    /// Describes that no specific value for the menu button was set.
    Default,
}
