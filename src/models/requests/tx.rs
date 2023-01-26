use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{Model, RequestMethod};

/// The tx method retrieves information on a single transaction.
///
/// See Tx:
/// `<https://xrpl.org/tx.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Tx<'a> {
    // TODO
    // #[serde(rename(serialize = "tx_json", deserialize = "transaction"))]
    // transaction,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// If true, return transaction data and metadata as binary
    /// serialized to hexadecimal strings. If false, return
    /// transaction data and metadata as JSON. The default is false.
    pub binary: Option<bool>,
    /// Use this with max_ledger to specify a range of up to 1000
    /// ledger indexes, starting with this ledger (inclusive). If
    /// the server cannot find the transaction, it confirms whether
    /// it was able to search all the ledgers in this range.
    pub min_ledger: Option<u32>,
    /// Use this with min_ledger to specify a range of up to 1000
    /// ledger indexes, ending with this ledger (inclusive). If the
    /// server cannot find the transaction, it confirms whether it
    /// was able to search all the ledgers in the requested range.
    pub max_ledger: Option<u32>,
    /// The request method.
    #[serde(default = "RequestMethod::tx")]
    pub command: RequestMethod,
}

impl Default for Tx<'static> {
    fn default() -> Self {
        Tx {
            id: None,
            binary: None,
            min_ledger: None,
            max_ledger: None,
            command: RequestMethod::Tx,
        }
    }
}

impl Model for Tx<'static> {}
