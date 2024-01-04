use alloc::borrow::Cow;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::amount::XRPAmount;
use crate::models::transactions::{CommonFields, NoFlags};
use crate::models::{
    model::Model,
    transactions::{Transaction, TransactionType},
};

use super::{Memo, Signer};

/// An AccountDelete transaction deletes an account and any objects it
/// owns in the XRP Ledger, if possible, sending the account's remaining
/// XRP to a specified destination account. See Deletion of Accounts for
/// the requirements to delete an account.
///
/// See AccountDelete:
/// `<https://xrpl.org/accountdelete.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AccountDelete<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(flatten)]
    pub common_fields: CommonFields<'a, NoFlags>,
    // The custom fields for the AccountDelete model.
    //
    // See AccountDelete fields:
    // `<https://xrpl.org/accountdelete.html#accountdelete-fields>`
    /// The address of an account to receive any leftover XRP after
    /// deleting the sending account. Must be a funded account in
    /// the ledger, and must not be the sending account.
    pub destination: Cow<'a, str>,
    /// Arbitrary destination tag that identifies a hosted
    /// recipient or other information for the recipient
    /// of the deleted account's leftover XRP.
    pub destination_tag: Option<u32>,
}

impl<'a> Model for AccountDelete<'a> {}

impl<'a> Transaction<NoFlags> for AccountDelete<'a> {
    fn get_transaction_type(&self) -> TransactionType {
        self.common_fields.get_transaction_type()
    }
}

impl<'a> AccountDelete<'a> {
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
        destination: Cow<'a, str>,
        destination_tag: Option<u32>,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                account,
                transaction_type: TransactionType::AccountDelete,
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
            destination,
            destination_tag,
        }
    }
}

#[cfg(test)]
mod test_serde {
    use super::*;

    #[test]
    fn test_serialize() {
        let default_txn = AccountDelete::new(
            "rWYkbWkCeg8dP6rXALnjgZSjjLyih5NXm".into(),
            None,
            Some("2000000".into()),
            None,
            None,
            Some(2470665),
            None,
            None,
            None,
            "rPT1Sjq2YGrBMTttX4GZHjKu9dyfzbpAYe".into(),
            Some(13),
        );
        let default_json = r#"{"TransactionType":"AccountDelete","Account":"rWYkbWkCeg8dP6rXALnjgZSjjLyih5NXm","Fee":"2000000","Sequence":2470665,"Destination":"rPT1Sjq2YGrBMTttX4GZHjKu9dyfzbpAYe","DestinationTag":13}"#;

        let txn_as_string = serde_json::to_string(&default_txn).unwrap();
        let txn_json = txn_as_string.as_str();

        assert_eq!(txn_json, default_json);
    }

    #[test]
    fn test_deserialize() {
        let default_txn = AccountDelete::new(
            "rWYkbWkCeg8dP6rXALnjgZSjjLyih5NXm".into(),
            None,
            Some("2000000".into()),
            None,
            None,
            Some(2470665),
            None,
            None,
            None,
            "rPT1Sjq2YGrBMTttX4GZHjKu9dyfzbpAYe".into(),
            Some(13),
        );
        let default_json = r#"{"TransactionType":"AccountDelete","Account":"rWYkbWkCeg8dP6rXALnjgZSjjLyih5NXm","Destination":"rPT1Sjq2YGrBMTttX4GZHjKu9dyfzbpAYe","DestinationTag":13,"Fee":"2000000","Sequence":2470665}"#;

        let txn_as_obj: AccountDelete = serde_json::from_str(default_json).unwrap();

        assert_eq!(txn_as_obj, default_txn);
    }
}
