//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{InputSticker, MaskPosition, True, UserId};

impl_payload! {
    @[multipart = sticker]
    /// Use this method to add a new sticker to a set created by the bot. Animated stickers can be added to animated sticker sets and only to them. Animated sticker sets can have up to 50 stickers. Static sticker sets can have up to 120 stickers. Returns _True_ on success.
    #[derive(Debug, Clone, Serialize)]
    pub AddStickerToSet (AddStickerToSetSetters) => True {
        required {
            /// User identifier of sticker file owner
            pub user_id: UserId,
            /// Sticker set name
            pub name: String [into],
            /// **PNG** or **TGS** image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed 512px, and either width or height must be exactly 512px. Pass a _file\_id_ as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. [More info on Sending Files »]
            ///
            /// [More info on Sending Files »]: crate::media_types::InputFile
            #[serde(flatten)]
            pub sticker: InputSticker,
            /// One or more emoji corresponding to the sticker
            pub emojis: String [into],
        }
        optional {
            /// A JSON-serialized object for position where the mask should be placed on faces
            pub mask_position: MaskPosition,
        }
    }
}
