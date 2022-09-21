//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::WebhookInfo;

impl_payload! {
    /// Use this method to get current webhook status. Requires no parameters. On success, returns a [`WebhookInfo`] object. If the bot is using [`GetUpdates`], will return an object with the _url_ field empty.
    ///
    /// [`WebhookInfo`]: crate::types::WebhookInfo
    /// [`GetUpdates`]: crate::payloads::GetUpdates
    #[derive(Debug, PartialEq, Eq, Hash, Default, Clone, Serialize)]
    pub GetWebhookInfo (GetWebhookInfoSetters) => WebhookInfo {

    }
}
