pub mod exceptions;

use crate::{
    asynch::{
        account::get_next_valid_seq_number,
        clients::{AsyncClient, CommonFields},
        ledger::{get_fee, get_latest_validated_ledger_sequence},
        transaction::exceptions::XRPLSignTransactionException,
    },
    core::{
        addresscodec::{is_valid_xaddress, xaddress_to_classic_address},
        binarycodec::{encode, encode_for_signing},
        keypairs::sign as keypairs_sign,
    },
    models::{
        amount::XRPAmount,
        exceptions::XRPLModelException,
        requests::{ServerState, Submit},
        results::{ServerState as ServerStateResult, Submit as SubmitResult},
        transactions::{Transaction, TransactionType, XRPLTransactionFieldException},
        Model,
    },
    utils::transactions::{
        get_transaction_field_value, set_transaction_field_value, validate_transaction_has_field,
    },
    wallet::Wallet,
    Err,
};

use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::{borrow::Cow, dbg};
use anyhow::Result;
use core::convert::TryInto;
use core::fmt::Debug;
use exceptions::XRPLTransactionException;
use rust_decimal::Decimal;
use serde::Serialize;
use serde::{de::DeserializeOwned, Deserialize};
use strum::IntoEnumIterator;

const OWNER_RESERVE: &str = "2000000"; // 2 XRP
const RESTRICTED_NETWORKS: u16 = 1024;
const REQUIRED_NETWORKID_VERSION: &str = "1.11.0";
const LEDGER_OFFSET: u8 = 20;

pub async fn autofill<'a, 'b, F, T, C>(
    transaction: &mut T,
    client: &'b C,
    signers_count: Option<u8>,
) -> Result<()>
where
    T: Transaction<'a, F> + Model + Clone,
    F: IntoEnumIterator + Serialize + Debug + PartialEq,
    C: AsyncClient,
{
    let txn = transaction.clone();
    let txn_common_fields = transaction.get_mut_common_fields();
    let common_fields = client.get_common_fields().await?;
    if txn_common_fields.network_id.is_none() && txn_needs_network_id(common_fields.clone())? {
        txn_common_fields.network_id = common_fields.network_id;
    }
    if txn_common_fields.sequence.is_none() {
        txn_common_fields.sequence =
            Some(get_next_valid_seq_number(txn_common_fields.account.clone(), client, None).await?);
    }
    if txn_common_fields.fee.is_none() {
        txn_common_fields.fee =
            Some(calculate_fee_per_transaction_type(&txn, Some(client), signers_count).await?);
    }
    if txn_common_fields.last_ledger_sequence.is_none() {
        let ledger_sequence = get_latest_validated_ledger_sequence(client).await?;
        txn_common_fields.last_ledger_sequence = Some(ledger_sequence + LEDGER_OFFSET as u32);
    }

    Ok(())
}

pub async fn calculate_fee_per_transaction_type<'a, 'b, 'c, T, F, C>(
    transaction: &T,
    client: Option<&'b C>,
    signers_count: Option<u8>,
) -> Result<XRPAmount<'c>>
where
    T: Transaction<'a, F>,
    F: IntoEnumIterator + Serialize + Debug + PartialEq,
    C: AsyncClient,
{
    let mut net_fee = XRPAmount::from("10");
    let base_fee;
    if let Some(client) = client {
        net_fee = get_fee(client, None, None).await?;
        base_fee = match transaction.get_transaction_type() {
            TransactionType::EscrowFinish => calculate_base_fee_for_escrow_finish(
                net_fee.clone(),
                transaction
                    .get_field_value("fulfillment")?
                    .map(|fulfillment| fulfillment.into()),
            )?,
            // TODO: same for TransactionType::AMMCreate
            TransactionType::AccountDelete => get_owner_reserve_from_response(client).await?,
            _ => net_fee.clone(),
        };
    } else {
        base_fee = match transaction.get_transaction_type() {
            TransactionType::EscrowFinish => calculate_base_fee_for_escrow_finish(
                net_fee.clone(),
                transaction
                    .get_field_value("fulfillment")?
                    .map(|fulfillment| fulfillment.into()),
            )?,
            // TODO: same for TransactionType::AMMCreate
            TransactionType::AccountDelete => XRPAmount::from(OWNER_RESERVE),
            _ => net_fee.clone(),
        };
    }
    let mut base_fee_decimal: Decimal = base_fee.try_into()?;
    if let Some(signers_count) = signers_count {
        let net_fee_decimal: Decimal = net_fee.try_into()?;
        let signer_count_fee_decimal: Decimal = (1 + signers_count).into();
        base_fee_decimal += &(net_fee_decimal * signer_count_fee_decimal);
    }

    Ok(base_fee_decimal.ceil().into())
}

async fn get_owner_reserve_from_response(client: &impl AsyncClient) -> Result<XRPAmount<'_>> {
    let owner_reserve_response = client.request(ServerState::new(None).into()).await?;
    match owner_reserve_response
        .try_into_result::<ServerStateResult<'_>>()?
        .state
        .validated_ledger
    {
        Some(validated_ledger) => Ok(validated_ledger.reserve_base),
        None => Err!(XRPLModelException::MissingField("validated_ledger")),
    }
}

fn calculate_base_fee_for_escrow_finish<'a: 'b, 'b>(
    net_fee: XRPAmount<'a>,
    fulfillment: Option<Cow<str>>,
) -> Result<XRPAmount<'b>> {
    if let Some(fulfillment) = fulfillment {
        calculate_based_on_fulfillment(fulfillment, net_fee)
    } else {
        Ok(net_fee)
    }
}

fn calculate_based_on_fulfillment<'a>(
    fulfillment: Cow<str>,
    net_fee: XRPAmount<'_>,
) -> Result<XRPAmount<'a>> {
    let fulfillment_bytes: Vec<u8> = fulfillment.chars().map(|c| c as u8).collect();
    let net_fee_f64: f64 = net_fee.try_into()?;
    let base_fee_string =
        (net_fee_f64 * (33.0 + (fulfillment_bytes.len() as f64 / 16.0))).to_string();
    let base_fee: XRPAmount = base_fee_string.into();
    let base_fee_decimal: Decimal = base_fee.try_into()?;

    Ok(base_fee_decimal.ceil().into())
}

fn txn_needs_network_id(common_fields: CommonFields<'_>) -> Result<bool> {
    let is_higher_restricted_networks = if let Some(network_id) = common_fields.network_id {
        network_id > RESTRICTED_NETWORKS as u32
    } else {
        false
    };
    if let Some(build_version) = common_fields.build_version {
        match is_not_later_rippled_version(REQUIRED_NETWORKID_VERSION.into(), build_version.into())
        {
            Ok(is_not_later_rippled_version) => {
                Ok(is_higher_restricted_networks && is_not_later_rippled_version)
            }
            Err(e) => Err!(e),
        }
    } else {
        Ok(false)
    }
}

fn is_not_later_rippled_version<'a>(
    source: String,
    target: String,
) -> Result<bool, XRPLTransactionException<'a>> {
    if source == target {
        Ok(true)
    } else {
        let source_decomp = source
            .split('.')
            .map(|i| i.to_string())
            .collect::<Vec<String>>();
        let target_decomp = target
            .split('.')
            .map(|i| i.to_string())
            .collect::<Vec<String>>();
        let (source_major, source_minor) = (
            source_decomp[0]
                .parse::<u8>()
                .map_err(XRPLTransactionException::ParseRippledVersionError)?,
            source_decomp[1]
                .parse::<u8>()
                .map_err(XRPLTransactionException::ParseRippledVersionError)?,
        );
        let (target_major, target_minor) = (
            target_decomp[0]
                .parse::<u8>()
                .map_err(XRPLTransactionException::ParseRippledVersionError)?,
            target_decomp[1]
                .parse::<u8>()
                .map_err(XRPLTransactionException::ParseRippledVersionError)?,
        );
        if source_major != target_major {
            Ok(source_major < target_major)
        } else if source_minor != target_minor {
            Ok(source_minor < target_minor)
        } else {
            let source_patch = source_decomp[2]
                .split('-')
                .map(|i| i.to_string())
                .collect::<Vec<String>>();
            let target_patch = target_decomp[2]
                .split('-')
                .map(|i| i.to_string())
                .collect::<Vec<String>>();
            let source_patch_version = source_patch[0]
                .parse::<u8>()
                .map_err(XRPLTransactionException::ParseRippledVersionError)?;
            let target_patch_version = target_patch[0]
                .parse::<u8>()
                .map_err(XRPLTransactionException::ParseRippledVersionError)?;
            if source_patch_version != target_patch_version {
                Ok(source_patch_version < target_patch_version)
            } else if source_patch.len() != target_patch.len() {
                Ok(source_patch.len() < target_patch.len())
            } else if source_patch.len() == 2 {
                if source_patch[1].chars().next().ok_or(
                    XRPLTransactionException::InvalidRippledVersion("source patch version".into()),
                )? != target_patch[1].chars().next().ok_or(
                    XRPLTransactionException::InvalidRippledVersion("target patch version".into()),
                )? {
                    Ok(source_patch[1] < target_patch[1])
                } else if source_patch[1].starts_with('b') {
                    Ok(&source_patch[1][1..] < &target_patch[1][1..])
                } else {
                    Ok(&source_patch[1][2..] < &target_patch[1][2..])
                }
            } else {
                Ok(false)
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
enum AccountFieldType {
    Account,
    Destination,
}

pub async fn sign_and_submit<'a, 'b, T, F, C>(
    transaction: &mut T,
    client: &'b C,
    wallet: &Wallet,
    autofill: bool,
    check_fee: bool,
) -> Result<SubmitResult<'a>>
where
    F: IntoEnumIterator + Serialize + Debug + PartialEq,
    T: Transaction<'a, F> + Model + Serialize + DeserializeOwned + Clone + Debug,
    C: AsyncClient,
{
    if autofill {
        autofill_and_sign(transaction, client, wallet, check_fee).await?;
    } else {
        if check_fee {
            check_txn_fee(transaction, client).await?;
        }
        sign(transaction, wallet, false)?;
    }
    submit(transaction, client).await
}

pub fn sign<'a, T, F>(transaction: &mut T, wallet: &Wallet, _multisign: bool) -> Result<()>
where
    F: IntoEnumIterator + Serialize + Debug + PartialEq,
    T: Transaction<'a, F> + Serialize + DeserializeOwned + Clone + Debug,
{
    prepare_transaction(transaction, wallet)?;
    let serialized_for_signing = encode_for_signing(transaction)?;
    let serialized_bytes = hex::decode(serialized_for_signing).unwrap(); // TODO: handle unwrap
    let signature = keypairs_sign(&serialized_bytes, &wallet.private_key).unwrap(); // TODO: handle unwrap
    transaction.get_mut_common_fields().txn_signature = Some(signature.into());

    Ok(())
}

pub async fn autofill_and_sign<'a, 'b, T, F, C>(
    transaction: &mut T,
    client: &'b C,
    wallet: &Wallet,
    check_fee: bool,
) -> Result<()>
where
    F: IntoEnumIterator + Serialize + Debug + PartialEq,
    T: Transaction<'a, F> + Model + Serialize + DeserializeOwned + Clone + Debug,
    C: AsyncClient,
{
    if check_fee {
        check_txn_fee(transaction, client).await?;
    }
    autofill(transaction, client, None).await?;
    sign(transaction, wallet, false)?;

    Ok(())
}

pub async fn submit<'a, T, F, C>(transaction: &T, client: &C) -> Result<SubmitResult<'a>>
where
    F: IntoEnumIterator + Serialize + Debug + PartialEq,
    T: Transaction<'a, F> + Serialize + DeserializeOwned + Clone + Debug,
    C: AsyncClient,
{
    let txn_blob = encode(transaction)?;
    let req = Submit::new(None, txn_blob.into(), None);
    let res = client.request(req.into()).await?;
    match res.try_into_result::<SubmitResult<'_>>() {
        Ok(value) => {
            let submit_result = SubmitResult::from(value);
            Ok(submit_result)
        }
        Err(e) => Err!(e),
    }
}

async fn check_txn_fee<'a, 'b, T, F, C>(transaction: &mut T, client: &'b C) -> Result<()>
where
    F: IntoEnumIterator + Serialize + Debug + PartialEq,
    T: Transaction<'a, F> + Model + Serialize + DeserializeOwned + Clone,
    C: AsyncClient,
{
    // max of xrp_to_drops(0.1) and calculate_fee_per_transaction_type
    let expected_fee = XRPAmount::from("100000")
        .max(calculate_fee_per_transaction_type(transaction, Some(client), None).await?);
    let transaction_fee = transaction
        .get_common_fields()
        .fee
        .clone()
        .unwrap_or(XRPAmount::from("0"));
    if transaction_fee > expected_fee {
        return Err!(XRPLSignTransactionException::FeeTooHigh(
            transaction_fee.try_into()?
        ));
    }
    Ok(())
}

fn prepare_transaction<'a, T, F>(transaction: &mut T, wallet: &Wallet) -> Result<()>
where
    F: IntoEnumIterator + Serialize + Debug + PartialEq,
    T: Transaction<'a, F> + Serialize + DeserializeOwned + Clone,
{
    let commond_fields = transaction.get_mut_common_fields();
    commond_fields.signing_pub_key = Some(wallet.public_key.clone().into());

    validate_account_xaddress(transaction, AccountFieldType::Account)?;
    if validate_transaction_has_field(transaction, "Destination").is_ok() {
        validate_account_xaddress(transaction, AccountFieldType::Destination)?;
    }

    let _ = convert_to_classic_address(transaction, "Unauthorize");
    let _ = convert_to_classic_address(transaction, "Authorize");
    // EscrowCancel, EscrowFinish
    let _ = convert_to_classic_address(transaction, "Owner");
    // SetRegularKey

    let _ = convert_to_classic_address(transaction, "RegularKey");

    Ok(())
}

fn validate_account_xaddress<'a, T, F>(
    prepared_transaction: &mut T,
    account_field: AccountFieldType,
) -> Result<()>
where
    F: IntoEnumIterator + Serialize + Debug + PartialEq,
    T: Transaction<'a, F> + Serialize + DeserializeOwned + Clone,
{
    let (account_field_name, tag_field_name) = match serde_json::to_string(&account_field) {
        Ok(name) => {
            let name_str = name.as_str().trim();
            if name_str == "\"Account\"" {
                ("Account", "SourceTag")
            } else if name_str == "\"Destination\"" {
                ("Destination", "DestinationTag")
            } else {
                return Err!(XRPLTransactionFieldException::UnknownAccountField(name_str));
            }
        }
        Err(error) => return Err!(error),
    };
    let account_address = match account_field {
        AccountFieldType::Account => prepared_transaction.get_common_fields().account.clone(),
        AccountFieldType::Destination => {
            get_transaction_field_value(prepared_transaction, "Destination")?
        }
    };

    if is_valid_xaddress(&account_address) {
        let (address, tag, _) = match xaddress_to_classic_address(&account_address) {
            Ok(t) => t,
            Err(error) => return Err!(error),
        };
        validate_transaction_has_field(prepared_transaction, &account_field_name)?;
        set_transaction_field_value(prepared_transaction, &account_field_name, address)?;

        if validate_transaction_has_field(prepared_transaction, &tag_field_name).is_ok()
            && get_transaction_field_value(prepared_transaction, &tag_field_name).unwrap_or(Some(0))
                != tag
        {
            Err!(XRPLSignTransactionException::TagFieldMismatch(
                &tag_field_name
            ))
        } else {
            set_transaction_field_value(prepared_transaction, &tag_field_name, tag)?;

            Ok(())
        }
    } else {
        Ok(())
    }
}

fn convert_to_classic_address<'a, T, F>(transaction: &mut T, field_name: &str) -> Result<()>
where
    F: IntoEnumIterator + Serialize + Debug + PartialEq,
    T: Transaction<'a, F> + Serialize + DeserializeOwned + Clone,
{
    let address = get_transaction_field_value::<F, _, String>(transaction, field_name)?;
    if is_valid_xaddress(&address) {
        let classic_address = match xaddress_to_classic_address(&address) {
            Ok(t) => t.0,
            Err(error) => return Err!(error),
        };
        set_transaction_field_value(transaction, field_name, classic_address)
    } else {
        Ok(())
    }
}

#[cfg(all(feature = "websocket-std", feature = "std", not(feature = "websocket")))]
#[cfg(test)]
mod test_autofill {
    use super::autofill;
    use crate::{
        asynch::clients::{AsyncWebsocketClient, SingleExecutorMutex},
        models::{
            amount::{IssuedCurrencyAmount, XRPAmount},
            transactions::{OfferCreate, Transaction},
        },
    };
    use anyhow::Result;

    #[tokio::test]
    async fn test_autofill_txn() -> Result<()> {
        let mut txn = OfferCreate::new(
            "r9mhdWo1NXVZr2pDnCtC1xwxE85kFtSzYR".into(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            XRPAmount::from("1000000").into(),
            IssuedCurrencyAmount::new(
                "USD".into(),
                "rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq".into(),
                "0.3".into(),
            )
            .into(),
            None,
            None,
        );
        let client = AsyncWebsocketClient::<SingleExecutorMutex, _>::open(
            "wss://testnet.xrpl-labs.com/".parse().unwrap(),
        )
        .await
        .unwrap();
        autofill(&mut txn, &client, None).await?;

        assert!(txn.get_common_fields().network_id.is_none());
        assert!(txn.get_common_fields().sequence.is_some());
        assert!(txn.get_common_fields().fee.is_some());
        assert!(txn.get_common_fields().last_ledger_sequence.is_some());

        Ok(())
    }
}

#[cfg(all(feature = "websocket-std", feature = "std", not(feature = "websocket")))]
#[cfg(test)]
mod test_sign {
    use alloc::borrow::Cow;

    use crate::{
        asynch::{
            clients::{AsyncWebsocketClient, SingleExecutorMutex},
            transaction::{autofill_and_sign, sign},
        },
        models::transactions::{AccountSet, Transaction},
        wallet::Wallet,
    };

    #[test]
    fn test_sign() {
        let wallet = Wallet::new("sEdT7wHTCLzDG7ueaw4hroSTBvH7Mk5", 0).unwrap();
        let mut payment = AccountSet::new(
            Cow::from(wallet.classic_address.clone()),
            None,
            Some("10".into()),
            None,
            None,
            None,
            Some(227234),
            None,
            None,
            None,
            None,
            Some("6578616d706c652e636f6d".into()), // "example.com"
            None,
            None,
            None,
            None,
            None,
            None,
        );
        sign(&mut payment, &wallet, false).unwrap();
        let expected_signature: Cow<str> =
            "B310792432B0242C2542C2B46CA234C87F4AE3FFC33226797AF72A92D9295ED20BD05A85D0\
            C13760B653AE9B8C0D74B9BBD310B09524F63B41D1776E7F2BB609"
                .into();
        let actual_signature = payment.get_common_fields().txn_signature.as_ref().unwrap();
        assert_eq!(expected_signature, *actual_signature);
    }

    #[tokio::test]
    async fn test_autofill_and_sign() {
        let wallet = Wallet::new("sEdT7wHTCLzDG7ueaw4hroSTBvH7Mk5", 0).unwrap();
        let mut payment = AccountSet::new(
            Cow::from(wallet.classic_address.clone()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some("6578616d706c652e636f6d".into()), // "example.com"
            None,
            None,
            None,
            None,
            None,
            None,
        );
        let client = AsyncWebsocketClient::<SingleExecutorMutex, _>::open(
            "wss://testnet.xrpl-labs.com/".parse().unwrap(),
        )
        .await
        .unwrap();
        autofill_and_sign(&mut payment, &client, &wallet, true)
            .await
            .unwrap();
        assert!(payment.get_common_fields().sequence.is_some());
        assert!(payment.get_common_fields().txn_signature.is_some());
    }
}
