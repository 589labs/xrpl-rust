use alloc::borrow::Cow;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::amount::XRPAmount;
use crate::models::transactions::{CommonFields, Memo, Signer};
use crate::models::NoFlags;
use crate::models::{
    model::Model,
    transactions::{Transaction, TransactionType},
};

/// See SetFee:
/// `<https://xrpl.org/setfee.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SetFee<'a> {
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
    /// The custom fields for the SetFee model.
    ///
    /// See SetFee fields:
    /// `<https://xrpl.org/setfee.html#setfee-fields>`
    pub base_fee: XRPAmount<'a>,
    pub reference_fee_units: u32,
    pub reserve_base: u32,
    pub reserve_increment: u32,
    pub ledger_sequence: u32,
}

impl<'a> Model for SetFee<'a> {}

impl<'a> Transaction<'a, NoFlags> for SetFee<'a> {
    fn get_transaction_type(&self) -> TransactionType {
        self.common_fields.transaction_type.clone()
    }

    fn as_common_fields(&'a self) -> &'a CommonFields<'a, NoFlags> {
        &self.common_fields
    }

    fn as_mut_common_fields(&'a mut self) -> &'a mut CommonFields<'a, NoFlags> {
        &mut self.common_fields
    }
}

impl<'a> SetFee<'a> {
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
        base_fee: XRPAmount<'a>,
        reference_fee_units: u32,
        reserve_base: u32,
        reserve_increment: u32,
        ledger_sequence: u32,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                account,
                transaction_type: TransactionType::SetFee,
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
            base_fee,
            reference_fee_units,
            reserve_base,
            reserve_increment,
            ledger_sequence,
        }
    }
}
