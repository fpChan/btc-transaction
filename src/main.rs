mod btc_transaction;
mod regtest;
mod testnet;

use crate::btc_transaction::{build_tx_hex, BTCNetwork, TxType};
use crate::regtest::RegtestConf;
use crate::testnet::TestnetConf;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let tx_hex = build_tx_hex(TxType::P2PKH);
    println!("encode tx :{}", tx_hex);

    // let network = TestnetConf::default();
    let network = RegtestConf::default();
    network.broadcast_tx(tx_hex).await?;

    Ok(())
}
