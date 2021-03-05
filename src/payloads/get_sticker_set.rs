// This file is auto generated by `cg` <https://github.com/teloxide/cg> (24572cd + local changes).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::StickerSet;

impl_payload! {
    /// Use this method to get a sticker set. On success, a StickerSet object is returned.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub GetStickerSet (GetStickerSetSetters) => StickerSet {
        required {
            /// Name of the sticker set
            pub name: String [into],
        }
    }
}
