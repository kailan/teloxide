use std::net::IpAddr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::AllowedUpdate;

/// Contains information about the current status of a webhook.
///
/// [The official docs](https://core.telegram.org/bots/api#webhookinfo).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct WebhookInfo {
    /// Webhook URL, `None` if webhook is not set up.
    #[serde(with = "crate::option_url_from_string")]
    pub url: Option<reqwest::Url>,

    /// `true`, if a custom certificate was provided for webhook certificate
    /// checks.
    pub has_custom_certificate: bool,

    /// Number of updates awaiting delivery.
    pub pending_update_count: u32,

    /// Currently used webhook IP address.
    pub ip_address: Option<IpAddr>,

    /// Time of the most recent error that happened when trying to
    /// deliver an update via webhook.
    #[serde(default, with = "crate::serde_opt_date_from_unix_timestamp")]
    pub last_error_date: Option<DateTime<Utc>>,

    /// Error message in human-readable format for the most recent error that
    /// happened when trying to deliver an update via webhook.
    pub last_error_message: Option<String>,

    /// Time of the most recent error that happened when trying to synchronize
    /// available updates with Telegram data-centers.
    #[serde(default, with = "crate::serde_opt_date_from_unix_timestamp")]
    pub last_synchronization_error_date: Option<DateTime<Utc>>,

    /// Maximum allowed number of simultaneous HTTPS connections to the webhook
    /// for update delivery.
    pub max_connections: Option<u32>,

    /// A list of update types the bot is subscribed to. Defaults to all update
    /// types.
    pub allowed_updates: Option<Vec<AllowedUpdate>>,
}

// TODO[KB]: change this test so it doesn't depend on teloxide-core
// Regression test for <https://github.com/teloxide/teloxide-core/pull/166>
// #[test]
// fn empty_url() {
//     let json = r#"{"url":"","has_custom_certificate":false,"pending_update_count":0,"allowed_updates":["message"]}"#;
//     let actual: WebhookInfo = serde_json::from_str(json).unwrap();
//     let expected = WebhookInfo {
//         url: None,
//         has_custom_certificate: false,
//         pending_update_count: 0,
//         ip_address: None,
//         last_error_date: None,
//         last_error_message: None,
//         last_synchronization_error_date: None,
//         max_connections: None,
//         allowed_updates: Some(vec![AllowedUpdate::Message]),
//     };

//     assert_eq!(actual, expected);

//     let json = r#"{"ok":true,"result":{"url":"","has_custom_certificate":false,"pending_update_count":0,"allowed_updates":["message"]}}"#;
//     serde_json::from_str::<crate::net::TelegramResponse<WebhookInfo>>(json).unwrap();
// }
