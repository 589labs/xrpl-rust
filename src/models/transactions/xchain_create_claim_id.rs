use alloc::{borrow::Cow, vec::Vec};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    core::addresscodec::is_valid_classic_address,
    models::{
        transactions::exceptions::XRPLXChainCreateClaimIDException, FlagCollection, Model, NoFlags,
        XChainBridge, XRPAmount,
    },
    Err,
};

use super::{CommonFields, Memo, Signer, Transaction, TransactionType};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[skip_serializing_none]
pub struct XChainCreateClaimID<'a> {
    #[serde(flatten)]
    pub common_fields: CommonFields<'a, NoFlags>,
    pub other_chain_source: Cow<'a, str>,
    pub signature_reward: Cow<'a, str>,
    #[serde(rename = "XChainBridge")]
    pub xchain_bridge: XChainBridge<'a>,
}

impl Model for XChainCreateClaimID<'_> {
    fn get_errors(&self) -> Result<()> {
        self.get_other_chain_source_is_invalid_error()
    }
}

impl<'a> Transaction<'a, NoFlags> for XChainCreateClaimID<'a> {
    fn get_transaction_type(&self) -> super::TransactionType {
        TransactionType::XChainCreateClaimID
    }

    fn get_common_fields(&self) -> &CommonFields<'_, NoFlags> {
        &self.common_fields
    }

    fn get_mut_common_fields(&mut self) -> &mut CommonFields<'a, NoFlags> {
        &mut self.common_fields
    }
}

impl<'a> XChainCreateClaimID<'a> {
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
        other_chain_source: Cow<'a, str>,
        signature_reward: Cow<'a, str>,
        xchain_bridge: XChainBridge<'a>,
    ) -> XChainCreateClaimID<'a> {
        XChainCreateClaimID {
            common_fields: CommonFields {
                account,
                transaction_type: TransactionType::XChainCreateClaimID,
                account_txn_id,
                fee,
                flags: FlagCollection::default(),
                last_ledger_sequence,
                memos,
                sequence,
                signers,
                source_tag,
                ticket_sequence,
                network_id: None,
                signing_pub_key: None,
                txn_signature: None,
            },
            other_chain_source,
            signature_reward,
            xchain_bridge,
        }
    }

    fn get_other_chain_source_is_invalid_error(&self) -> Result<()> {
        if !is_valid_classic_address(self.other_chain_source.as_ref()) {
            Err!(XRPLXChainCreateClaimIDException::OtherChainSourceIsInvalid)
        } else {
            Ok(())
        }
    }
}
