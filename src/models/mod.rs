//! Top-level modules for the models package.

pub mod exceptions;
pub mod requests;
pub mod utils;

pub use requests::*;

use alloc::borrow::Cow;
use alloc::borrow::Cow::Borrowed;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::AsRefStr;
use strum_macros::{Display, EnumIter};

/// Represents the different options for the `method`
/// field in a request.
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Display, EnumIter)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum RequestMethod {
    // Account methods
    AccountChannels,
    AccountCurrencies,
    AccountInfo,
    AccountLines,
    AccountObjects,
    AccountOffers,
    AccountTx,
    GatewayBalances,
    NoRippleCheck,

    // Transaction methods
    Sign,
    SignFor,
    Submit,
    SubmitMultisigned,
    TransactionEntry,
    Tx,

    // Channel methods
    ChannelAuthorize,
    ChannelVerify,

    // Path methods
    BookOffers,
    DepositAuthorized,
    PathFind,
    RipplePathFind,

    // Ledger methods
    Ledger,
    LedgerClosed,
    LedgerCurrent,
    LedgerData,
    LedgerEntry,

    // Subscription methods
    Subscribe,
    Unsubscribe,

    // Server info methods
    Fee,
    Manifest,
    ServerInfo,
    ServerState,

    // Utility methods
    Ping,
    Random,
}

/// Transactions of the TrustSet type support additional values
/// in the Flags field. This enum represents those options.
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Display, AsRefStr)]
pub enum TrustSetFlag {
    TfSetAuth,
    TfSetNoRipple,
    TfClearNoRipple,
    TfSetFreeze,
    TfClearFreeze,
}

/// Represents the object types that an AccountObjects
/// Request can ask for.
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum AccountObjectType {
    Check,
    DepositPreauth,
    Escrow,
    Offer,
    PaymentChannel,
    SignerList,
    State,
    Ticket,
}

/// Specifies an amount in an issued currency.
///
/// See Specifying Currency Amounts:
/// `<https://xrpl.org/currency-formats.html#specifying-currency-amounts>`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Currency {
    /// Specifies an amount in an issued currency.
    IssuedCurrency {
        amount: Option<Cow<'static, str>>,
        currency: Cow<'static, str>,
        issuer: Cow<'static, str>,
    },
    /// Specifies an amount in XRP.
    Xrp {
        amount: Option<Cow<'static, str>>,
        currency: Cow<'static, str>,
    },
}

/// Enum containing the different Transaction types.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(tag = "transaction_type")]
pub enum TransactionType {
    AccountDelete,
    AccountSet,
    CheckCancel,
    CheckCash,
    CheckCreate,
    DepositPreauth,
    EscrowCancel,
    EscrowCreate,
    EscrowFinish,
    OfferCancel,
    OfferCreate,
    Payment,
    PaymentChannelClaim,
    PaymentChannelCreate,
    PaymentChannelFund,
    SetRegularKey,
    SignerListSet,
    TrustSet,

    // Psuedo-Transaction types,
    EnableAmendment,
    SetFee,
    UNLModify,
}

/// Enum representing the options for the address role in
/// a NoRippleCheckRequest.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[serde(tag = "role")]
pub enum NoRippleCheckRole {
    Gateway,
    User,
}

/// There are three different modes, or sub-commands, of
/// the path_find command. Specify which one you want with
/// the subcommand parameter:
/// * create - Start sending pathfinding information
/// * close - Stop sending pathfinding information
/// * status - Info on the currently-open pathfinding request
///
/// See Path Find:
/// `<https://xrpl.org/path_find.html>`
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[serde(tag = "subcommand")]
pub enum PathFindSubcommand {
    Create,
    Close,
    Status,
}

/// Represents possible values of the streams query param
/// for subscribe.
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[serde(tag = "streams")]
pub enum StreamParameter {
    Consensus,
    Ledger,
    Manifests,
    PeerStatus,
    Transactions,
    TransactionsProposed,
    Server,
    Validations,
}

/// Required fields for requesting a DepositPreauth if not
/// querying by object ID.
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositPreauth<'a> {
    owner: &'a str,
    authorized: &'a str,
}

/// Required fields for requesting a DirectoryNode if not
/// querying by object ID.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Directory<'a> {
    owner: &'a str,
    dir_root: &'a str,
    sub_index: Option<u8>,
}

/// Required fields for requesting a Escrow if not querying
/// by object ID.
#[derive(Debug, Serialize, Deserialize)]
pub struct Escrow<'a> {
    owner: &'a str,
    seq: u64,
}

/// Required fields for requesting a Escrow if not querying
/// by object ID.
#[derive(Debug, Serialize, Deserialize)]
pub struct Offer<'a> {
    account: &'a str,
    seq: u64,
}

/// Required fields for requesting a RippleState.
#[derive(Debug, Serialize, Deserialize)]
pub struct RippleState<'a> {
    account: &'a str,
    currency: &'a str,
}

/// Required fields for requesting a Ticket, if not
/// querying by object ID.
#[derive(Debug, Serialize, Deserialize)]
pub struct Ticket<'a> {
    owner: &'a str,
    ticket_sequence: u64,
}

/// A PathStep represents an individual step along a Path.
#[derive(Debug, Serialize, Deserialize)]
pub struct PathStep<'a> {
    account: Option<&'a str>,
    currency: Option<&'a str>,
    issuer: Option<&'a str>,
    r#type: Option<u8>,
    type_hex: Option<&'a str>,
}

/// An arbitrary piece of data attached to a transaction. A
/// transaction can have multiple Memo objects as an array
/// in the Memos field.
///
/// Must contain one or more of `memo_data`, `memo_format`,
/// and `memo_type`.
///
/// See Memos Field:
/// `<https://xrpl.org/transaction-common-fields.html#memos-field>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct Memo<'a> {
    memo_data: Option<&'a str>,
    memo_format: Option<&'a str>,
    memo_type: Option<&'a str>,
}

/// One Signer in a multi-signature. A multi-signed transaction
/// can have an array of up to 8 Signers, each contributing a
/// signature, in the Signers field.
///
/// See Signers Field:
/// `<https://xrpl.org/transaction-common-fields.html#signers-field>`
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct Signer<'a> {
    account: &'a str,
    txn_signature: &'a str,
    signing_pub_key: &'a str,
}

/// Format for elements in the `books` array for Subscribe only.
///
/// See Subscribe:
/// `<https://xrpl.org/subscribe.html#subscribe>`
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct SubscribeBook<'a> {
    taker_gets: Currency,
    taker_pays: Currency,
    taker: &'a str,
    #[serde(default = "default_false")]
    snapshot: Option<bool>,
    #[serde(default = "default_false")]
    both: Option<bool>,
}

/// Format for elements in the `books` array for Unsubscribe only.
///
/// See Unsubscribe:
/// `<https://xrpl.org/unsubscribe.html>`
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct UnsubscribeBook {
    taker_gets: Currency,
    taker_pays: Currency,
    #[serde(default = "default_false")]
    both: Option<bool>,
}

/// The base fields for all transaction models.
///
/// See Transaction Types:
/// `<https://xrpl.org/transaction-types.html>`
///
/// See Transaction Common Fields:
/// `<https://xrpl.org/transaction-common-fields.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct TransactionFields<'a> {
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u64>,
    last_ledger_sequence: Option<u64>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
}

/// Returns a Currency as XRP for the currency, without a value.
pub fn default_xrp_currency() -> Currency {
    Currency::Xrp {
        amount: None,
        currency: Borrowed("XRP"),
    }
}

/// For use with serde defaults.
fn default_true() -> Option<bool> {
    Some(true)
}

/// For use with serde defaults.
fn default_false() -> Option<bool> {
    Some(false)
}

/// For use with serde defaults.
fn default_limit_200() -> Option<u16> {
    Some(200)
}

/// For use with serde defaults.
fn default_limit_300() -> Option<u16> {
    Some(300)
}

/// For use with serde defaults.
fn default_fee_mult_max() -> Option<u32> {
    Some(10)
}

/// For use with serde defaults.
fn default_fee_div_max() -> Option<u32> {
    Some(1)
}

/// Allows creation of a Model object based on a JSON-like
/// dictionary of keys in the JSON format used by the binary
/// codec, or an actual JSON string representing the same data.
pub trait FromXRPL<T> {
    fn from_xrpl(value: T) -> Self;
}

/// For use with serde defaults.
impl TrustSetFlag {
    fn tf_set_auth() -> u32 {
        0x00010000
    }
    fn tf_set_no_ripple() -> u32 {
        0x00020000
    }
    fn tf_clear_no_ripple() -> u32 {
        0x00040000
    }
    fn tf_set_freeze() -> u32 {
        0x00100000
    }
    fn tf_clear_freeze() -> u32 {
        0x00200000
    }
}

/// For use with serde defaults.
/// TODO Find a better way
impl RequestMethod {
    fn account_channels() -> Self {
        RequestMethod::AccountChannels
    }
    fn account_currencies() -> Self {
        RequestMethod::AccountCurrencies
    }
    fn account_info() -> Self {
        RequestMethod::AccountInfo
    }
    fn account_lines() -> Self {
        RequestMethod::AccountLines
    }
    fn account_objects() -> Self {
        RequestMethod::AccountObjects
    }
    fn account_offers() -> Self {
        RequestMethod::AccountOffers
    }
    fn account_tx() -> Self {
        RequestMethod::AccountTx
    }
    fn book_offers() -> Self {
        RequestMethod::BookOffers
    }
    fn channel_authorize() -> Self {
        RequestMethod::ChannelAuthorize
    }
    fn channel_verify() -> Self {
        RequestMethod::ChannelVerify
    }
    fn deposit_authorization() -> Self {
        RequestMethod::DepositAuthorized
    }
    fn fee() -> Self {
        RequestMethod::Fee
    }
    fn ledger_closed() -> Self {
        RequestMethod::LedgerClosed
    }
    fn ledger_current() -> Self {
        RequestMethod::LedgerCurrent
    }
    fn ledger_data() -> Self {
        RequestMethod::LedgerData
    }
    fn ledger_entry() -> Self {
        RequestMethod::LedgerEntry
    }
    fn ledger() -> Self {
        RequestMethod::Ledger
    }
    fn manifest() -> Self {
        RequestMethod::Manifest
    }
    fn no_ripple_check() -> Self {
        RequestMethod::NoRippleCheck
    }
    fn path_find() -> Self {
        RequestMethod::PathFind
    }
    fn ripple_path_find() -> Self {
        RequestMethod::RipplePathFind
    }
    fn ping() -> Self {
        RequestMethod::Ping
    }
    fn random() -> Self {
        RequestMethod::Random
    }
    fn server_info() -> Self {
        RequestMethod::ServerInfo
    }
    fn server_state() -> Self {
        RequestMethod::ServerState
    }
    fn submit() -> Self {
        RequestMethod::Submit
    }
    fn sign_for() -> Self {
        RequestMethod::SignFor
    }
    fn sign() -> Self {
        RequestMethod::Sign
    }
    fn submit_multisigned() -> Self {
        RequestMethod::SubmitMultisigned
    }
    fn subscribe() -> Self {
        RequestMethod::Subscribe
    }
    fn unsubscribe() -> Self {
        RequestMethod::Unsubscribe
    }
    fn transaction_entry() -> Self {
        RequestMethod::TransactionEntry
    }
    fn tx() -> Self {
        RequestMethod::Tx
    }
}
