use serde::{Deserialize, Serialize};

use crate::{ShippingAddress, User};

/// This object contains information about an incoming shipping query.
///
/// [The official docs](https://core.telegram.org/bots/api#shippingquery).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ShippingQuery {
    /// Unique query identifier.
    pub id: String,

    /// User who sent the query.
    pub from: User,

    /// Bot specified invoice payload.
    pub invoice_payload: String,

    /// User specified shipping address.
    pub shipping_address: ShippingAddress,
}
