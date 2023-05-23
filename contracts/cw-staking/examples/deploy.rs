use abstract_boot::boot_core::{
    instantiate_daemon_env, networks::NetworkInfo, DaemonOptionsBuilder, *,
};
use abstract_boot::{AnsHost, ApiDeployer, VCExecFns, VersionControl};
use abstract_cw_staking_api::boot::CwStakingApi;
use abstract_cw_staking_api::CW_STAKING;
use abstract_sdk::core::objects::module::{Module, ModuleInfo, ModuleVersion};
use abstract_sdk::core::{api, ANS_HOST, VERSION_CONTROL};
use boot_core::networks::{NetworkKind, ChainInfo};
use cosmwasm_std::{Addr, Empty};
use semver::Version;
use std::sync::Arc;
use tokio::runtime::Runtime;

const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const PION_1: NetworkInfo = NetworkInfo {
    kind: NetworkKind::Testnet,
    id: "pion-1",
    gas_denom: "untrn",
    gas_price: 0.000,
    grpc_urls: &["http://grpc-palvus.pion-1.ntrn.tech:80"],
    chain_info: ChainInfo { chain_id: "neutron", pub_address_prefix: "neutron", coin_type: 118 },
    lcd_url: Some("https://rest-palvus.pion-1.ntrn.tech"),
    fcd_url: None,
};

fn deploy_cw_staking(
) -> anyhow::Result<()> {
    let module_version: Version = CONTRACT_VERSION.parse().unwrap();

    let rt = Arc::new(Runtime::new()?);
    let options = DaemonOptionsBuilder::default().network(PION_1).build();
    let (_sender, chain) = instantiate_daemon_env(&rt, options?)?;

    let version_control = VersionControl::new(VERSION_CONTROL, chain.clone());

    let ans_host = AnsHost::new(ANS_HOST, chain.clone());

    
        log::info!("Uploading Cw staking");
        // Upload and deploy with the version
        let mut cw_staking = CwStakingApi::new(CW_STAKING, chain);

        cw_staking.deploy(module_version, Empty {})?;
    

    Ok(())
}

use clap::Parser;

fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();

    use dotenv::dotenv;

    deploy_cw_staking()
}
