use abstract_boot::ApiDeployer;

use abstract_boot::boot_core::networks::{parse_network, NetworkInfo};
use abstract_boot::boot_core::*;
use abstract_dex_api::boot::DexApi;
use abstract_dex_api::msg::DexInstantiateMsg;
use abstract_dex_api::EXCHANGE;
use boot_core::networks::{NetworkKind, ChainInfo};
use cosmwasm_std::Decimal;
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

fn deploy_dex() -> anyhow::Result<()> {
    let version: Version = CONTRACT_VERSION.parse().unwrap();
    let rt = Arc::new(Runtime::new()?);
    let options = DaemonOptionsBuilder::default().network(PION_1).build();
    let (_sender, chain) = instantiate_daemon_env(&rt, options?)?;
    let mut dex = DexApi::new(EXCHANGE, chain);
    dex.deploy(
        version,
        DexInstantiateMsg {
            swap_fee: Decimal::percent(1),
            recipient_os: 0,
        },
    )?;
    Ok(())
}

use clap::Parser;

#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Network Id to deploy on
    #[arg(short, long)]
    network_id: String,
}

fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();

    use dotenv::dotenv;

 
    deploy_dex()
}
