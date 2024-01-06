use alloc::borrow::Cow;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::amount::XRPAmount;
use crate::models::NoFlags;
use crate::models::{
    model::Model,
    transactions::{Memo, Signer, Transaction, TransactionType},
};

use super::CommonFields;

/// Removes an Offer object from the XRP Ledger.
///
/// See OfferCancel:
/// `<https://xrpl.org/offercancel.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct OfferCancel<'a> {
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(flatten)]
    pub common_fields: CommonFields<'a, NoFlags>,
    // The custom fields for the OfferCancel model.
    //
    // See OfferCancel fields:
    // `<https://xrpl.org/offercancel.html#offercancel-fields>`
    /// The sequence number (or Ticket number) of a previous OfferCreate transaction.
    /// If specified, cancel any offer object in the ledger that was created by that
    /// transaction. It is not considered an error if the offer specified does not exist.
    pub offer_sequence: u32,
}

impl<'a> Model for OfferCancel<'a> {}

impl<'a> Transaction<NoFlags> for OfferCancel<'a> {
    fn get_transaction_type(&self) -> TransactionType {
        self.common_fields.transaction_type.clone()
    }
}

impl<'a> OfferCancel<'a> {
    pub fn new(
        account: Cow<'a, str>,
        account_txn_id: Option<Cow<'a, str>>,
        fee: Option<XRPAmount<'a>>,
        last_ledger_sequence: Option<u32>,
        memos: Option<Vec<Memo>>,
        sequence: Option<u32>,
        signers: Option<Vec<Signer<'a>>>,
        source_tag: Option<u32>,
        ticket_sequence: Option<u32>,
        offer_sequence: u32,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                account,
                transaction_type: TransactionType::OfferCancel,
                account_txn_id,
                fee,
                flags: None,
                last_ledger_sequence,
                memos,
                sequence,
                signers,
                source_tag,
                ticket_sequence,
            },
            offer_sequence,
        }
    }
}

#[cfg(test)]
mod test_serde {
    use super::*;

    #[test]
    fn test_serialize() {
        let default_txn = OfferCancel::new(
            "ra5nK24KXen9AHvsdFTKHSANinZseWnPcX".into(),
            None,
            Some("12".into()),
            Some(7108629),
            None,
            Some(7),
            None,
            None,
            None,
            6,
        );
        let default_json = r#"{"TransactionType":"OfferCancel","Account":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","Fee":"12","Sequence":7,"LastLedgerSequence":7108629,"OfferSequence":6}"#;

        let txn_as_string = serde_json::to_string(&default_txn).unwrap();
        let txn_json = txn_as_string.as_str();

        assert_eq!(txn_json, default_json);
    }

    #[test]
    fn test_deserialize() {
        let default_txn = OfferCancel::new(
            "ra5nK24KXen9AHvsdFTKHSANinZseWnPcX".into(),
            Some("12".into()),
            None,
            Some(7108629),
            None,
            Some(7),
            None,
            None,
            None,
            6,
        );
        let default_json = r#"{"TransactionType":"OfferCancel","Account":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","Fee":"12","LastLedgerSequence":7108629,"OfferSequence":6,"Sequence":7}"#;

        let txn_as_obj: OfferCancel = serde_json::from_str(default_json).unwrap();

        assert_eq!(txn_as_obj, default_txn);
    }
}
