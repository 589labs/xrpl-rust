use crate::Err;
use alloc::borrow::Cow;
use anyhow::Result;
use derive_new::new;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::requests::XRPLLedgerEntryException;
use crate::models::{requests::RequestMethod, Model};

use super::{CommonFields, Request};

/// Required fields for requesting a DepositPreauth if not
/// querying by object ID.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, new)]
pub struct DepositPreauth<'a> {
    pub authorized: Cow<'a, str>,
    pub owner: Cow<'a, str>,
}

/// Required fields for requesting a DirectoryNode if not
/// querying by object ID.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, new)]
pub struct Directory<'a> {
    pub dir_root: Cow<'a, str>,
    pub owner: Cow<'a, str>,
    pub sub_index: Option<u8>,
}

/// Required fields for requesting a Escrow if not querying
/// by object ID.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, new)]
pub struct Escrow<'a> {
    pub owner: Cow<'a, str>,
    pub seq: u64,
}

/// Required fields for requesting a Escrow if not querying
/// by object ID.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, new)]
pub struct Offer<'a> {
    pub account: Cow<'a, str>,
    pub seq: u64,
}

/// Required fields for requesting a Ticket, if not
/// querying by object ID.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, new)]
pub struct Ticket<'a> {
    pub owner: Cow<'a, str>,
    pub ticket_sequence: u64,
}

/// Required fields for requesting a RippleState.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, new)]
pub struct RippleState<'a> {
    pub account: Cow<'a, str>,
    pub currency: Cow<'a, str>,
}

/// The ledger_entry method returns a single ledger object
/// from the XRP Ledger in its raw format. See ledger formats
/// for information on the different types of objects you can
/// retrieve.
///
/// See Ledger Formats:
/// `<https://xrpl.org/ledger-data-formats.html#ledger-data-formats>`
///
/// See Ledger Entry:
/// `<https://xrpl.org/ledger_entry.html#ledger_entry>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct LedgerEntry<'a> {
    /// The common fields shared by all requests.
    #[serde(flatten)]
    pub common_fields: CommonFields<'a>,
    pub account_root: Option<Cow<'a, str>>,
    /// If true, return the requested ledger object's contents as a
    /// hex string in the XRP Ledger's binary format. Otherwise, return
    /// data in JSON format. The default is false.
    pub binary: Option<bool>,
    pub check: Option<Cow<'a, str>>,
    pub deposit_preauth: Option<DepositPreauth<'a>>,
    pub directory: Option<Directory<'a>>,
    pub escrow: Option<Escrow<'a>>,
    pub index: Option<Cow<'a, str>>,
    /// A 20-byte hex string for the ledger version to use.
    pub ledger_hash: Option<Cow<'a, str>>,
    /// The ledger index of the ledger to use, or a shortcut string
    /// (e.g. "validated" or "closed" or "current") to choose a ledger
    /// automatically.
    pub ledger_index: Option<Cow<'a, str>>,
    pub offer: Option<Offer<'a>>,
    pub payment_channel: Option<Cow<'a, str>>,
    pub ripple_state: Option<RippleState<'a>>,
    pub ticket: Option<Ticket<'a>>,
}

impl<'a: 'static> Model for LedgerEntry<'a> {
    fn get_errors(&self) -> Result<()> {
        match self._get_field_error() {
            Err(error) => Err!(error),
            Ok(_no_error) => Ok(()),
        }
    }
}

impl<'a> LedgerEntryError for LedgerEntry<'a> {
    fn _get_field_error(&self) -> Result<(), XRPLLedgerEntryException> {
        let mut signing_methods: u32 = 0;
        for method in [
            self.index.clone(),
            self.account_root.clone(),
            self.check.clone(),
        ] {
            if method.is_some() {
                signing_methods += 1
            }
        }
        if self.directory.is_some() {
            signing_methods += 1
        }
        if self.offer.is_some() {
            signing_methods += 1
        }
        if self.ripple_state.is_some() {
            signing_methods += 1
        }
        if self.escrow.is_some() {
            signing_methods += 1
        }
        if self.payment_channel.is_some() {
            signing_methods += 1
        }
        if self.deposit_preauth.is_some() {
            signing_methods += 1
        }
        if self.ticket.is_some() {
            signing_methods += 1
        }
        if signing_methods != 1 {
            Err(XRPLLedgerEntryException::DefineExactlyOneOf {
                field1: "index".into(),
                field2: "account_root".into(),
                field3: "check".into(),
                field4: "directory".into(),
                field5: "offer".into(),
                field6: "ripple_state".into(),
                field7: "escrow".into(),
                field8: "payment_channel".into(),
                field9: "deposit_preauth".into(),
                field10: "ticket".into(),
                resource: "".into(),
            })
        } else {
            Ok(())
        }
    }
}

impl<'a> Request for LedgerEntry<'a> {
    fn get_command(&self) -> RequestMethod {
        self.common_fields.command.clone()
    }
}

impl<'a> LedgerEntry<'a> {
    pub fn new(
        id: Option<Cow<'a, str>>,
        account_root: Option<Cow<'a, str>>,
        binary: Option<bool>,
        check: Option<Cow<'a, str>>,
        deposit_preauth: Option<DepositPreauth<'a>>,
        directory: Option<Directory<'a>>,
        escrow: Option<Escrow<'a>>,
        index: Option<Cow<'a, str>>,
        ledger_hash: Option<Cow<'a, str>>,
        ledger_index: Option<Cow<'a, str>>,
        offer: Option<Offer<'a>>,
        payment_channel: Option<Cow<'a, str>>,
        ripple_state: Option<RippleState<'a>>,
        ticket: Option<Ticket<'a>>,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                command: RequestMethod::LedgerEntry,
                id,
            },
            index,
            account_root,
            check,
            payment_channel,
            deposit_preauth,
            directory,
            escrow,
            offer,
            ripple_state,
            ticket,
            binary,
            ledger_hash,
            ledger_index,
        }
    }
}

pub trait LedgerEntryError {
    fn _get_field_error(&self) -> Result<(), XRPLLedgerEntryException>;
}

#[cfg(test)]
mod test_ledger_entry_errors {
    use super::Offer;
    use crate::models::requests::XRPLLedgerEntryException;
    use crate::models::Model;
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_fields_error() {
        let ledger_entry = LedgerEntry::new(
            None,
            Some("rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn".into()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(Offer {
                account: "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn".into(),
                seq: 359,
            }),
            None,
            None,
            None,
        );
        let _expected = XRPLLedgerEntryException::DefineExactlyOneOf {
            field1: "index".into(),
            field2: "account_root".into(),
            field3: "check".into(),
            field4: "directory".into(),
            field5: "offer".into(),
            field6: "ripple_state".into(),
            field7: "escrow".into(),
            field8: "payment_channel".into(),
            field9: "deposit_preauth".into(),
            field10: "ticket".into(),
            resource: "".into(),
        };
        assert_eq!(
            ledger_entry.validate().unwrap_err().to_string().as_str(),
            "Define one of: `index`, `account_root`, `check`, `directory`, `offer`, `ripple_state`, `escrow`, `payment_channel`, `deposit_preauth`, `ticket`. Define exactly one of them. For more information see: "
        );
    }
}
