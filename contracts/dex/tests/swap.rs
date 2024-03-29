mod common;

use abstract_boot::boot_core::Deploy;
use abstract_boot::boot_core::{instantiate_default_mock_env, ContractInstance};
use abstract_boot::{Abstract, AbstractAccount, ApiDeployer};
use abstract_dex_api::{boot::DexApi, msg::DexInstantiateMsg, EXCHANGE};
use boot_core::{CallAs, Mock};
use boot_cw_plus::{Cw20ExecuteMsgFns, Cw20QueryMsgFns};
use common::create_default_os;
use cosmwasm_std::{coin, Addr, Decimal, Empty};

use speculoos::*;
use wyndex_bundle::{EUR, RAW_TOKEN, USD, WYNDEX_OWNER};

fn setup_mock() -> anyhow::Result<(
    Mock,
    wyndex_bundle::WynDex,
    DexApi<Mock>,
    AbstractAccount<Mock>,
)> {
    let sender = Addr::unchecked(common::ROOT_USER);
    let (_state, chain) = instantiate_default_mock_env(&sender)?;

    let deployment = Abstract::deploy_on(chain.clone(), "1.0.0".parse()?)?;
    let wyndex = wyndex_bundle::WynDex::deploy_on(chain.clone(), Empty {})?;

    let _root_os = create_default_os(&deployment.account_factory)?;
    let mut exchange_api = DexApi::new(EXCHANGE, chain.clone());

    exchange_api.deploy(
        "1.0.0".parse()?,
        DexInstantiateMsg {
            swap_fee: Decimal::percent(1),
            recipient_os: 0,
        },
    )?;

    let os = create_default_os(&deployment.account_factory)?;

    // mint to proxy
    chain.set_balance(&os.proxy.address()?, vec![coin(10_000, EUR)])?;
    // install exchange on OS
    os.manager.install_module(EXCHANGE, &Empty {})?;
    // load exchange data into type
    exchange_api.set_address(&Addr::unchecked(
        os.manager.module_info(EXCHANGE)?.unwrap().address,
    ));

    Ok((chain, wyndex, exchange_api, os))
}

#[test]
fn swap_native() -> anyhow::Result<()> {
    let (chain, _, exchange_api, os) = setup_mock()?;
    let proxy_addr = os.proxy.address()?;

    // swap 100 EUR to USD
    exchange_api.swap((EUR, 100), USD, wyndex_bundle::WYNDEX.into())?;

    // check balances
    let eur_balance = chain.query_balance(&proxy_addr, EUR)?;
    assert_that!(eur_balance.u128()).is_equal_to(9_900);

    let usd_balance = chain.query_balance(&proxy_addr, USD)?;
    assert_that!(usd_balance.u128()).is_equal_to(98);

    // assert that OS 0 received the swap fee
    let os0_proxy = AbstractAccount::new(chain.clone(), Some(0))
        .proxy
        .address()?;
    let os0_eur_balance = chain.query_balance(&os0_proxy, EUR)?;
    assert_that!(os0_eur_balance.u128()).is_equal_to(1);

    Ok(())
}

#[test]
fn swap_raw() -> anyhow::Result<()> {
    let (chain, wyndex, exchange_api, os) = setup_mock()?;
    let proxy_addr = os.proxy.address()?;

    // trnasfer raw
    let owner = Addr::unchecked(WYNDEX_OWNER);
    wyndex
        .raw_token
        .call_as(&owner)
        .transfer(10_000u128.into(), (&proxy_addr).to_string())?;

    // swap 100 RAW to EUR
    exchange_api.swap((RAW_TOKEN, 100), EUR, wyndex_bundle::WYNDEX.into())?;

    // check balances
    let raw_balance = wyndex.raw_token.balance(proxy_addr.to_string())?;
    assert_that!(raw_balance.balance.u128()).is_equal_to(9_900);

    let eur_balance = chain.query_balance(&proxy_addr, EUR)?;
    assert_that!(eur_balance.u128()).is_equal_to(10098);

    // assert that OS 0 received the swap fee
    let os0_proxy = AbstractAccount::new(chain.clone(), Some(0))
        .proxy
        .address()?;
    let os0_raw_balance = wyndex.raw_token.balance(os0_proxy.to_string())?;
    assert_that!(os0_raw_balance.balance.u128()).is_equal_to(1);

    Ok(())
}
