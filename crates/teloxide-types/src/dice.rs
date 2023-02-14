use serde::{Deserialize, Serialize};

use crate::DiceEmoji;

/// This object represents an animated emoji that displays a random value.
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dice {
    /// Emoji on which the dice throw animation is based.
    pub emoji: DiceEmoji,

    /// Value of the dice.
    ///
    /// 1-6 for [`DiceEmoji::Dice`] and [`DiceEmoji::Darts`], 1-5 for
    /// [`DiceEmoji::Basketball`].
    ///
    /// [`DiceEmoji::Dice`]: crate::DiceEmoji::Dice
    /// [`DiceEmoji::Darts`]:crate::DiceEmoji::Darts
    /// [`DiceEmoji::Basketball`]:crate::DiceEmoji::Basketball
    pub value: i32,
}
