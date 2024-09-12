use alloc::borrow::Cow;
use anyhow::Result;
use embassy_futures::block_on;

use crate::{
    asynch::{
        account::{
            does_account_exist as async_does_account_exist,
            get_account_root as async_get_account_root,
            get_latest_transaction as async_get_latest_transaction,
            get_next_valid_seq_number as async_get_next_valid_seq_number,
            get_xrp_balance as async_get_xrp_balance,
        },
        clients::XRPLClient,
    },
    models::{ledger::objects::AccountRoot, results::account_tx::AccountTx, XRPAmount},
};

pub fn does_account_exist<C>(
    address: Cow<'_, str>,
    client: &C,
    ledger_index: Option<Cow<'_, str>>,
) -> Result<bool>
where
    C: XRPLClient,
{
    block_on(async_does_account_exist(address, client, ledger_index))
}

pub fn get_next_valid_seq_number<C>(
    address: Cow<'_, str>,
    client: &C,
    ledger_index: Option<Cow<'_, str>>,
) -> Result<u32>
where
    C: XRPLClient,
{
    block_on(async_get_next_valid_seq_number(
        address,
        client,
        ledger_index,
    ))
}

pub fn get_xrp_balance<'a: 'b, 'b, C>(
    address: Cow<'a, str>,
    client: &C,
    ledger_index: Option<Cow<'a, str>>,
) -> Result<XRPAmount<'b>>
where
    C: XRPLClient,
{
    block_on(async_get_xrp_balance(address, client, ledger_index))
}

pub fn get_account_root<'a: 'b, 'b, C>(
    address: Cow<'a, str>,
    client: &C,
    ledger_index: Cow<'a, str>,
) -> Result<AccountRoot<'b>>
where
    C: XRPLClient,
{
    block_on(async_get_account_root(address, client, ledger_index))
}

pub fn get_latest_transaction<'a: 'b, 'b, C>(
    address: Cow<'a, str>,
    client: &C,
) -> Result<AccountTx<'b>>
where
    C: XRPLClient,
{
    block_on(async_get_latest_transaction(address, client))
}