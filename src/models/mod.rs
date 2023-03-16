//! Top-level modules for the models package.
//!
//! Order of models:
//! 1. Type of model
//! 2. Required common fields in alphabetical order
//! 3. Optional common fields in alphabetical order
//! 4. Required specific fields in alphabetical order
//! 5. Optional specific fields in alphabetical order

pub mod exceptions;
pub mod ledger;
pub mod model;
#[allow(clippy::too_many_arguments)]
pub mod requests;
#[allow(clippy::too_many_arguments)]
pub mod transactions;

pub mod response;
pub mod utils;

use derive_new::new;
pub use model::Model;
use serde::ser::SerializeMap;

use crate::_serde::currency_xrp;
use crate::_serde::HashMap;
use crate::serde_with_tag;

use alloc::borrow::Cow;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::AsRefStr;
use strum_macros::{Display, EnumIter};

use self::exceptions::{
    AccountSetException, ChannelAuthorizeException, CheckCashException, DepositPreauthException,
    EscrowCreateException, EscrowFinishException, LedgerEntryException,
    NFTokenAcceptOfferException, NFTokenCancelOfferException, NFTokenCreateOfferException,
    NFTokenMintException, PaymentException, SignAndSubmitException, SignException,
    SignForException, SignerListSetException, UNLModifyException,
};
use self::transactions::account_set::AccountSetFlag;
use self::transactions::nftoken_create_offer::NFTokenCreateOfferFlag;
use self::transactions::nftoken_mint::NFTokenMintFlag;
use self::transactions::offer_create::OfferCreateFlag;
use self::transactions::payment::PaymentFlag;
use self::transactions::payment_channel_claim::PaymentChannelClaimFlag;
use self::transactions::pseudo_transactions::enable_amendment::EnableAmendmentFlag;
use self::transactions::trust_set::TrustSetFlag;

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
    AccountNfts,
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
    NftBuyOffers,
    NftSellOffers,
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

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Display, AsRefStr)]
pub enum Flag {
    AccountSet(AccountSetFlag),
    NFTokenCreateOffer(NFTokenCreateOfferFlag),
    NFTokenMint(NFTokenMintFlag),
    OfferCreate(OfferCreateFlag),
    Payment(PaymentFlag),
    PaymentChannelClaim(PaymentChannelClaimFlag),
    TrustSet(TrustSetFlag),
    EnableAmendment(EnableAmendmentFlag),
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
    RippleState,
    Ticket,
}

/// Specifies a currency.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Currency {
    /// Specifies an issued currency.
    IssuedCurrency {
        currency: Cow<'static, str>,
        issuer: Cow<'static, str>,
    },
    /// Specifies XRP.
    #[serde(with = "currency_xrp")]
    Xrp,
}

impl Default for Currency {
    fn default() -> Self {
        Self::Xrp
    }
}

impl Currency {
    /// Check wether the defined currency is XRP.
    fn is_xrp(&self) -> bool {
        match self {
            Currency::IssuedCurrency {
                currency: _,
                issuer: _,
            } => false,
            Currency::Xrp => true,
        }
    }
}

/// Specifies a currency amount.
///
/// See Specifying Currency Amounts:
/// `<https://xrpl.org/currency-formats.html#specifying-currency-amounts>`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Amount {
    /// Specifies an amount in an issued currency.
    IssuedCurrency {
        currency: Cow<'static, str>,
        issuer: Cow<'static, str>,
        value: Cow<'static, str>,
    },
    /// Specifies an amount in XRP.
    Xrp(Cow<'static, str>),
}

impl Default for Amount {
    fn default() -> Self {
        Self::Xrp(Cow::Borrowed("()"))
    }
}

impl Amount {
    /// Returns the specified currency value as `u32`.
    fn get_value_as_u32(&self) -> u32 {
        match self {
            Amount::IssuedCurrency {
                currency: _,
                issuer: _,
                value,
            } => {
                let value_as_u32: u32 = value
                    .as_ref()
                    .parse()
                    .expect("Could not parse u32 from `value`");
                value_as_u32
            }
            Amount::Xrp(value) => {
                let value_as_u32: u32 = value
                    .as_ref()
                    .parse()
                    .expect("Could not parse u32 from `value`");
                value_as_u32
            }
        }
    }

    /// Check wether the defined currency amount is a XRP amount.
    fn is_xrp(&self) -> bool {
        match self {
            Amount::IssuedCurrency {
                currency: _,
                issuer: _,
                value: _,
            } => false,
            Amount::Xrp(_) => true,
        }
    }
}

/// Enum containing the different Transaction types.
#[derive(Debug, Clone, Serialize, Deserialize, Display, PartialEq, Eq)]
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
    NFTokenAcceptOffer,
    NFTokenBurn,
    NFTokenCancelOffer,
    NFTokenCreateOffer,
    NFTokenMint,
    OfferCancel,
    OfferCreate,
    Payment,
    PaymentChannelClaim,
    PaymentChannelCreate,
    PaymentChannelFund,
    SetRegularKey,
    SignerListSet,
    TicketCreate,
    TrustSet,

    // Psuedo-Transaction types,
    EnableAmendment,
    SetFee,
    UNLModify,
}

/// Represents possible values of the streams query param
/// for subscribe.
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
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

serde_with_tag! {
/// An arbitrary piece of data attached to a transaction. A
/// transaction can have multiple Memo objects as an array
/// in the Memos field.
///
/// Must contain one or more of `memo_data`, `memo_format`,
/// and `memo_type`.
///
/// See Memos Field:
/// `<https://xrpl.org/transaction-common-fields.html#memos-field>`
// `#[derive(Serialize)]` is defined in the macro
#[derive(Debug, PartialEq, Eq, Default, Clone, new)]
#[skip_serializing_none]
pub struct Memo<'a> {
    pub memo_data: Option<&'a str>,
    pub memo_format: Option<&'a str>,
    pub memo_type: Option<&'a str>,
}
}

/// A PathStep represents an individual step along a Path.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default, Clone, new)]
#[serde(rename_all = "PascalCase")]
pub struct PathStep<'a> {
    account: Option<&'a str>,
    currency: Option<&'a str>,
    issuer: Option<&'a str>,
    r#type: Option<u8>,
    type_hex: Option<&'a str>,
}

/// One Signer in a multi-signature. A multi-signed transaction
/// can have an array of up to 8 Signers, each contributing a
/// signature, in the Signers field.
///
/// See Signers Field:
/// `<https://xrpl.org/transaction-common-fields.html#signers-field>`
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default, Clone, new)]
#[serde(rename_all = "PascalCase")]
pub struct Signer<'a> {
    account: &'a str,
    txn_signature: &'a str,
    signing_pub_key: &'a str,
}

/// Returns a Currency as XRP for the currency, without a value.
fn default_xrp_currency() -> Currency {
    Currency::Xrp
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
    fn account_nfts() -> Self {
        RequestMethod::AccountNfts
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
    fn nft_buy_offers() -> Self {
        RequestMethod::NftBuyOffers
    }
    fn nft_sell_offers() -> Self {
        RequestMethod::NftSellOffers
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

/// Standard functions for transactions.
pub trait Transaction {
    fn has_flag(&self, flag: &Flag) -> bool {
        let _txn_flag = flag;
        false
    }

    fn get_transaction_type(&self) -> TransactionType;
}

pub trait AccountSetError {
    fn _get_tick_size_error(&self) -> Result<(), AccountSetException>;
    fn _get_transfer_rate_error(&self) -> Result<(), AccountSetException>;
    fn _get_domain_error(&self) -> Result<(), AccountSetException>;
    fn _get_clear_flag_error(&self) -> Result<(), AccountSetException>;
    fn _get_nftoken_minter_error(&self) -> Result<(), AccountSetException>;
}

pub trait CheckCashError {
    fn _get_amount_and_deliver_min_error(&self) -> Result<(), CheckCashException>;
}

pub trait DepositPreauthError {
    fn _get_authorize_and_unauthorize_error(&self) -> Result<(), DepositPreauthException>;
}

pub trait EscrowCreateError {
    fn _get_finish_after_error(&self) -> Result<(), EscrowCreateException>;
}

pub trait EscrowFinishError {
    fn _get_condition_and_fulfillment_error(&self) -> Result<(), EscrowFinishException>;
}

pub trait NFTokenAcceptOfferError {
    fn _get_brokered_mode_error(&self) -> Result<(), NFTokenAcceptOfferException>;
    fn _get_nftoken_broker_fee_error(&self) -> Result<(), NFTokenAcceptOfferException>;
}

pub trait NFTokenCancelOfferError {
    fn _get_nftoken_offers_error(&self) -> Result<(), NFTokenCancelOfferException>;
}

pub trait NFTokenCreateOfferError {
    fn _get_amount_error(&self) -> Result<(), NFTokenCreateOfferException>;
    fn _get_destination_error(&self) -> Result<(), NFTokenCreateOfferException>;
    fn _get_owner_error(&self) -> Result<(), NFTokenCreateOfferException>;
}

pub trait NFTokenMintError {
    fn _get_issuer_error(&self) -> Result<(), NFTokenMintException>;
    fn _get_transfer_fee_error(&self) -> Result<(), NFTokenMintException>;
    fn _get_uri_error(&self) -> Result<(), NFTokenMintException>;
}

pub trait PaymentError {
    fn _get_xrp_transaction_error(&self) -> Result<(), PaymentException>;
    fn _get_partial_payment_error(&self) -> Result<(), PaymentException>;
    fn _get_exchange_error(&self) -> Result<(), PaymentException>;
}

pub trait SignerListSetError {
    fn _get_signer_entries_error(&self) -> Result<(), SignerListSetException>;
    fn _get_signer_quorum_error(&self) -> Result<(), SignerListSetException>;
}

pub trait UNLModifyError {
    fn _get_unl_modify_error(&self) -> Result<(), UNLModifyException>;
}

pub trait ChannelAuthorizeError {
    fn _get_field_error(&self) -> Result<(), ChannelAuthorizeException>;
}

pub trait LedgerEntryError {
    fn _get_field_error(&self) -> Result<(), LedgerEntryException>;
}

pub trait SignAndSubmitError {
    fn _get_field_error(&self) -> Result<(), SignAndSubmitException>;
    fn _get_key_type_error(&self) -> Result<(), SignAndSubmitException>;
}

pub trait SignForError {
    fn _get_field_error(&self) -> Result<(), SignForException>;
    fn _get_key_type_error(&self) -> Result<(), SignForException>;
}

pub trait SignError {
    fn _get_field_error(&self) -> Result<(), SignException>;
    fn _get_key_type_error(&self) -> Result<(), SignException>;
}

/// For use with serde defaults.
/// TODO Find a better way
impl TransactionType {
    fn account_delete() -> Self {
        TransactionType::AccountDelete
    }
    fn account_set() -> Self {
        TransactionType::AccountSet
    }
    fn check_cancel() -> Self {
        TransactionType::CheckCancel
    }
    fn check_cash() -> Self {
        TransactionType::CheckCash
    }
    fn check_create() -> Self {
        TransactionType::CheckCreate
    }
    fn deposit_preauth() -> Self {
        TransactionType::DepositPreauth
    }
    fn escrow_cancel() -> Self {
        TransactionType::EscrowCancel
    }
    fn escrow_create() -> Self {
        TransactionType::EscrowCreate
    }
    fn escrow_finish() -> Self {
        TransactionType::EscrowFinish
    }
    fn nftoken_accept_offer() -> Self {
        TransactionType::NFTokenAcceptOffer
    }
    fn nftoken_burn() -> Self {
        TransactionType::NFTokenBurn
    }
    fn nftoken_cancel_offer() -> Self {
        TransactionType::NFTokenCancelOffer
    }
    fn nftoken_create_offer() -> Self {
        TransactionType::NFTokenCreateOffer
    }
    fn nftoken_mint() -> Self {
        TransactionType::NFTokenMint
    }
    fn offer_cancel() -> Self {
        TransactionType::OfferCancel
    }
    fn offer_create() -> Self {
        TransactionType::OfferCreate
    }
    fn payment() -> Self {
        TransactionType::Payment
    }
    fn payment_channel_claim() -> Self {
        TransactionType::PaymentChannelClaim
    }
    fn payment_channel_create() -> Self {
        TransactionType::PaymentChannelCreate
    }
    fn payment_channel_fund() -> Self {
        TransactionType::PaymentChannelFund
    }
    fn set_regular_key() -> Self {
        TransactionType::SetRegularKey
    }
    fn signer_list_set() -> Self {
        TransactionType::SignerListSet
    }
    fn ticket_create() -> Self {
        TransactionType::TicketCreate
    }
    fn trust_set() -> Self {
        TransactionType::TrustSet
    }
    fn enable_amendment() -> Self {
        TransactionType::EnableAmendment
    }
    fn set_fee() -> Self {
        TransactionType::SetFee
    }
    fn unl_modify() -> Self {
        TransactionType::UNLModify
    }
}
