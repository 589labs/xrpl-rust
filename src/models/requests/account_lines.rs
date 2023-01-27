use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{Model, RequestMethod};

/// This request returns information about an account's trust
/// lines, including balances in all non-XRP currencies and
/// assets. All information retrieved is relative to a particular
/// version of the ledger.
///
/// See Account Lines:
/// `<https://xrpl.org/account_lines.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountLines<'a> {
    /// A unique identifier for the account, most commonly the
    /// account's Address.
    pub account: &'a str,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// A 20-byte hex string for the ledger version to use.
    pub ledger_hash: Option<&'a str>,
    /// The ledger index of the ledger to use, or a shortcut
    /// string to choose a ledger automatically.
    pub ledger_index: Option<&'a str>,
    /// Limit the number of trust lines to retrieve. The server
    /// is not required to honor this value. Must be within the
    /// inclusive range 10 to 400.
    pub limit: Option<u16>,
    /// The Address of a second account. If provided, show only
    /// lines of trust connecting the two accounts.
    pub peer: Option<&'a str>,
    /// Value from a previous paginated response. Resume retrieving
    /// data where that response left off.
    pub marker: Option<u32>,
    /// The request method.
    #[serde(default = "RequestMethod::account_lines")]
    pub command: RequestMethod,
}

impl Default for AccountLines<'static> {
    fn default() -> Self {
        AccountLines {
            account: "",
            id: None,
            ledger_hash: None,
            ledger_index: None,
            limit: None,
            peer: None,
            marker: None,
            command: RequestMethod::AccountLines,
        }
    }
}

impl Model for AccountLines<'static> {}