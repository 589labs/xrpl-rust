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

/// Removes a NFToken object from the NFTokenPage in which it is being held,
/// effectively removing the token from the ledger (burning it).
///
/// See NFTokenBurn:
/// `<https://xrpl.org/nftokenburn.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct NFTokenBurn<'a> {
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
    // The custom fields for the NFTokenBurn model.
    //
    // See NFTokenBurn fields:
    // `<https://xrpl.org/nftokenburn.html#nftokenburn-fields>`
    #[serde(rename = "NFTokenID")]
    /// The NFToken to be removed by this transaction.
    pub nftoken_id: Cow<'a, str>,
    /// The owner of the NFToken to burn. Only used if that owner is
    /// different than the account sending this transaction. The
    /// issuer or authorized minter can use this field to burn NFTs
    /// that have the lsfBurnable flag enabled.
    pub owner: Option<Cow<'a, str>>,
}

impl<'a> Model for NFTokenBurn<'a> {}

impl<'a> Transaction<NoFlags> for NFTokenBurn<'a> {
    fn get_transaction_type(&self) -> TransactionType {
        self.common_fields.transaction_type.clone()
    }
}

impl<'a> NFTokenBurn<'a> {
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
        nftoken_id: Cow<'a, str>,
        owner: Option<Cow<'a, str>>,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                account,
                transaction_type: TransactionType::NFTokenBurn,
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
            nftoken_id,
            owner,
        }
    }
}

#[cfg(test)]
mod test_serde {
    use super::*;

    #[test]
    fn test_serialize() {
        let default_txn = NFTokenBurn::new(
            "rNCFjv8Ek5oDrNiMJ3pw6eLLFtMjZLJnf2".into(),
            None,
            Some("10".into()),
            None,
            None,
            None,
            None,
            None,
            None,
            "000B013A95F14B0044F78A264E41713C64B5F89242540EE208C3098E00000D65".into(),
            Some("rvYAfWj5gh67oV6fW32ZzP3Aw4Eubs59B".into()),
        );
        let default_json = r#"{"TransactionType":"NFTokenBurn","Account":"rNCFjv8Ek5oDrNiMJ3pw6eLLFtMjZLJnf2","Fee":"10","NFTokenID":"000B013A95F14B0044F78A264E41713C64B5F89242540EE208C3098E00000D65","Owner":"rvYAfWj5gh67oV6fW32ZzP3Aw4Eubs59B"}"#;

        let txn_as_string = serde_json::to_string(&default_txn).unwrap();
        let txn_json = txn_as_string.as_str();

        assert_eq!(txn_json, default_json);
    }

    #[test]
    fn test_deserialize() {
        let default_txn = NFTokenBurn::new(
            "rNCFjv8Ek5oDrNiMJ3pw6eLLFtMjZLJnf2".into(),
            None,
            Some("10".into()),
            None,
            None,
            None,
            None,
            None,
            None,
            "000B013A95F14B0044F78A264E41713C64B5F89242540EE208C3098E00000D65".into(),
            Some("rvYAfWj5gh67oV6fW32ZzP3Aw4Eubs59B".into()),
        );
        let default_json = r#"{"TransactionType":"NFTokenBurn","Account":"rNCFjv8Ek5oDrNiMJ3pw6eLLFtMjZLJnf2","Owner":"rvYAfWj5gh67oV6fW32ZzP3Aw4Eubs59B","Fee":"10","NFTokenID":"000B013A95F14B0044F78A264E41713C64B5F89242540EE208C3098E00000D65"}"#;

        let txn_as_obj: NFTokenBurn = serde_json::from_str(default_json).unwrap();

        assert_eq!(txn_as_obj, default_txn);
    }
}
