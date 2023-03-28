// use bindings::rmm01_portfolio::rmm01_portfolio::RMM01Portfolio;
use bindings::uniswap_v3_pool;
use ethers::abi::Abi;
use ethers::contract::Contract;
use ethers::prelude::Provider;
use ethers::providers::{Middleware, Http};
use ethers::types::{Address, Filter};
use eyre::Result;
use futures::stream::StreamExt;
use std::convert::TryFrom;
use std::env::var_os;
use std::sync::Arc;

const V3ETHUSDPOOL: &str = "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640";

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().expect(".env file not found");

    let client =
    Arc::new(Provider::try_from(var_os("RPC_URL").unwrap().into_string().unwrap()).unwrap());

    let contract_address: Address = V3ETHUSDPOOL.parse()?;
    let contract_abi: Abi = uniswap_v3_pool::UNISWAPV3POOL_ABI.clone();
    let i_portfolio = Contract::new(contract_address, contract_abi, client.clone());

    let event_filter = Filter {
        address: Some(ethers::types::ValueOrArray::Array(vec![
            i_portfolio.address()
        ])),
        topics: [None, None, None, None], // None for all topics
        ..Default::default()
    };

    let mut event_stream = client
        .watch(&event_filter)
        .await
        .expect("Failed to create event stream");

    while let Some(log) = event_stream.next().await {
        println!("Monitoring Events....");
        println!("Event data: {:?}", log);
    }
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