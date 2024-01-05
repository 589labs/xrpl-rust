use crate::Err;
use alloc::borrow::Cow;
use alloc::vec::Vec;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::transactions::XRPLEscrowFinishException;
use crate::models::NoFlags;
use crate::models::{
    amount::XRPAmount,
    model::Model,
    transactions::{Memo, Signer, Transaction, TransactionType},
};

use super::CommonFields;

/// Finishes an Escrow and delivers XRP from a held payment to the recipient.
///
/// See EscrowFinish:
/// `<https://xrpl.org/escrowfinish.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct EscrowFinish<'a> {
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
    /// The custom fields for the EscrowFinish model.
    ///
    /// See EscrowFinish fields:
    /// `<https://xrpl.org/escrowfinish.html#escrowfinish-fields>`
    pub owner: Cow<'a, str>,
    pub offer_sequence: u32,
    pub condition: Option<Cow<'a, str>>,
    pub fulfillment: Option<Cow<'a, str>>,
}

impl<'a: 'static> Model for EscrowFinish<'a> {
    fn get_errors(&self) -> Result<()> {
        match self._get_condition_and_fulfillment_error() {
            Ok(_) => Ok(()),
            Err(error) => Err!(error),
        }
    }
}

impl<'a> Transaction<NoFlags> for EscrowFinish<'a> {
    fn get_transaction_type(&self) -> TransactionType {
        self.common_fields.transaction_type.clone()
    }
}

impl<'a> EscrowFinishError for EscrowFinish<'a> {
    fn _get_condition_and_fulfillment_error(&self) -> Result<(), XRPLEscrowFinishException> {
        if (self.condition.is_some() && self.fulfillment.is_none())
            || (self.condition.is_none() && self.condition.is_some())
        {
            Err(XRPLEscrowFinishException::FieldRequiresField {
                field1: "condition".into(),
                field2: "fulfillment".into(),
                resource: "".into(),
            })
        } else {
            Ok(())
        }
    }
}

impl<'a> EscrowFinish<'a> {
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
        owner: Cow<'a, str>,
        offer_sequence: u32,
        condition: Option<Cow<'a, str>>,
        fulfillment: Option<Cow<'a, str>>,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                account,
                transaction_type: TransactionType::EscrowFinish,
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
            owner,
            offer_sequence,
            condition,
            fulfillment,
        }
    }
}

pub trait EscrowFinishError {
    fn _get_condition_and_fulfillment_error(&self) -> Result<(), XRPLEscrowFinishException>;
}

#[cfg(test)]
mod test_escrow_finish_errors {

    use crate::models::Model;
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_condition_and_fulfillment_error() {
        let escrow_finish = EscrowFinish::new(
            "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb".into(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb".into(),
            10,
            Some(
                "A0258020E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855810100"
                    .into(),
            ),
            None,
        );

        assert_eq!(
            escrow_finish.validate().unwrap_err().to_string().as_str(),
            "For the field `condition` to be defined it is required to also define the field `fulfillment`. For more information see: "
        );
    }
}

#[cfg(test)]
mod test_serde {
    use super::*;

    #[test]
    fn test_serialize() {
        let default_txn = EscrowFinish::new(
            "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn".into(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn".into(),
            7,
            Some(
                "A0258020E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855810100"
                    .into(),
            ),
            Some("A0028000".into()),
        );
        let default_json = r#"{"TransactionType":"EscrowFinish","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Owner":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","OfferSequence":7,"Condition":"A0258020E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855810100","Fulfillment":"A0028000"}"#;

        let txn_as_string = serde_json::to_string(&default_txn).unwrap();
        let txn_json = txn_as_string.as_str();

        assert_eq!(txn_json, default_json);
    }

    #[test]
    fn test_deserialize() {
        let default_txn = EscrowFinish::new(
            "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn".into(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn".into(),
            7,
            Some(
                "A0258020E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855810100"
                    .into(),
            ),
            Some("A0028000".into()),
        );
        let default_json = r#"{"TransactionType":"EscrowFinish","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Owner":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","OfferSequence":7,"Condition":"A0258020E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855810100","Fulfillment":"A0028000"}"#;

        let txn_as_obj: EscrowFinish = serde_json::from_str(default_json).unwrap();

        assert_eq!(txn_as_obj, default_txn);
    }
}
