use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{
    default_zero,
    exceptions::{DepositPreauthException, XRPLModelException, XRPLTransactionException},
    model::Model,
    DepositPreauthError, Memo, Signer, Transaction, TransactionType,
};

/// A DepositPreauth transaction gives another account pre-approval
/// to deliver payments to the sender of this transaction.
///
/// See DepositPreauth:
/// `<https://xrpl.org/depositpreauth.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct DepositPreauth<'a> {
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::deposit_preauth")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    pub account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    pub fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    pub sequence: Option<u32>,
    /// Highest ledger index this transaction can appear in.
    /// Specifying this field places a strict upper limit on how long
    /// the transaction can wait to be validated or rejected.
    /// See Reliable Transaction Submission for more details.
    pub last_ledger_sequence: Option<u32>,
    /// Hash value identifying another transaction. If provided, this
    /// transaction is only valid if the sending account's
    /// previously-sent transaction matches the provided hash.
    pub account_txn_id: Option<&'a str>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    pub signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    pub source_tag: Option<u32>,
    /// The sequence number of the ticket to use in place
    /// of a Sequence number. If this is provided, Sequence must
    /// be 0. Cannot be used with AccountTxnID.
    pub ticket_sequence: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    pub txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    #[serde(default = "default_zero")]
    flags: Option<u32>,
    /// Additional arbitrary information used to identify this transaction.
    pub memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    pub signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the DepositPreauth model.
    ///
    /// See DepositPreauth fields:
    /// `<https://xrpl.org/depositpreauth.html#depositpreauth-fields>`
    pub authorize: Option<&'a str>,
    pub unauthorize: Option<&'a str>,
}

impl Model for DepositPreauth<'static> {
    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self._get_authorize_and_unauthorize_error() {
            Ok(_no_error) => Ok(()),
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::DepositPreauthError(error),
            )),
        }
    }
}

impl From<&DepositPreauth<'static>> for u32 {
    fn from(_: &DepositPreauth<'static>) -> Self {
        0
    }
}

impl Transaction for DepositPreauth<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl DepositPreauthError for DepositPreauth<'static> {
    fn _get_authorize_and_unauthorize_error(&self) -> Result<(), DepositPreauthException> {
        match self.authorize.is_none() && self.unauthorize.is_none() {
            true => Err(DepositPreauthException::InvalidMustSetAuthorizeOrUnauthorize),
            false => match self.authorize.is_some() && self.unauthorize.is_some() {
                true => Err(DepositPreauthException::InvalidMustNotSetAuthorizeAndUnauthorize),
                false => Ok(()),
            },
        }
    }
}

#[cfg(test)]
mod test_deposit_preauth_exception {
    use crate::models::{
        exceptions::{DepositPreauthException, XRPLModelException, XRPLTransactionException},
        Model, TransactionType,
    };

    use super::DepositPreauth;

    #[test]
    fn test_authorize_and_unauthorize_error() {
        let mut deposit_preauth = DepositPreauth {
            transaction_type: TransactionType::DepositPreauth,
            account: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb",
            fee: None,
            sequence: None,
            last_ledger_sequence: None,
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: None,
            memos: None,
            signers: None,
            authorize: None,
            unauthorize: None,
        };
        let expected_error = XRPLModelException::XRPLTransactionError(
            XRPLTransactionException::DepositPreauthError(
                DepositPreauthException::InvalidMustSetAuthorizeOrUnauthorize,
            ),
        );
        assert_eq!(deposit_preauth.validate(), Err(expected_error));

        deposit_preauth.authorize = Some("rLSn6Z3T8uCxbcd1oxwfGQN1Fdn5CyGujK");
        deposit_preauth.unauthorize = Some("raQwCVAJVqjrVm1Nj5SFRcX8i22BhdC9WA");
        let expected_error = XRPLModelException::XRPLTransactionError(
            XRPLTransactionException::DepositPreauthError(
                DepositPreauthException::InvalidMustNotSetAuthorizeAndUnauthorize,
            ),
        );
        assert_eq!(deposit_preauth.validate(), Err(expected_error));
    }
}
