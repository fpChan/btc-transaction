mod btc_transaction;
mod regtest;
mod testnet;

use crate::btc_transaction::{build_tx_hex, BTCNetwork, TxType};
use crate::regtest::RegtestConf;
use crate::testnet::TestnetConf;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let out_point = "a8d7ecefd3f8c60d646a246498906f78d886bf3ceec6bc1daa595b70ddde7989:0";
    let tx_hex = build_tx_hex(out_point, 4999900000, 5000000000, TxType::P2PKH);
    println!("encode tx :{}", tx_hex);

    // let network = TestnetConf::default();
    let network = RegtestConf::default();
    network.broadcast_tx(tx_hex).await?;

    Ok(())
}
