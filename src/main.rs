// use bindings::rmm01_portfolio::rmm01_portfolio::RMM01Portfolio;
use ethers::{prelude::*};
use eyre::Result;
use ethers::{prelude::*, providers::Provider};
use std::{env::var_os, str::FromStr, sync::Arc};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().expect(".env file not found");

    let client = get_provider(RpcTypes::Goerli).await;

    let test_client = client.get_block(80).await.unwrap().unwrap();
    println!("Client Response block hash{:?}", test_client.state_root);

    Ok(())
}

#[derive(Debug, PartialEq)]
pub enum RpcTypes {
    Testnet,
    Devnet,
    Default,
    Goerli,
}

pub async fn get_provider(rpc_type: RpcTypes) -> Arc<Provider<Http>> {
    match rpc_type {
        RpcTypes::Testnet => {
            Arc::new(Provider::try_from("https://testnet.primitive.xyz/").unwrap())
        }
        RpcTypes::Devnet => Arc::new(Provider::try_from("http://localhost:8545").unwrap()),
        RpcTypes::Default => Provider::try_from(var_os("RPC_URL").unwrap().into_string().unwrap())
            .unwrap()
            .into(),
        RpcTypes::Goerli => Arc::new(Provider::try_from("https://rpc.ankr.com/eth_goerli").unwrap()),
    }
}