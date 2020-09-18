mod btc_explorer;
mod btc_transaction;

use crate::btc_explorer::{Bitaps, Blockcypher, BroadcastNode, Explorer};
use crate::btc_transaction::{get_tx_hex, TxType};
use reqwest::Error;
use std::{thread, time};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let broadcast_node = Bitaps {
        url: "https://api.bitaps.com/btc/testnet/".to_owned(),
    };
    let explorer = Blockcypher {
        url: "https://api.blockcypher.com/v1/btc/test3/".to_owned(),
    };

    let out_point = "9c0415c3bb7b788212bbef7c5200b7642abfc7cdd028320da988a6b489150fcd:0";

    let tx_hex = get_tx_hex(out_point, 12000, 13000, TxType::P2WPKH);
    println!("encode tx :{}", tx_hex);

    let tx_hash = broadcast_node.broadcast_tx(tx_hex).await?;

    thread::sleep(time::Duration::from_secs(60 * 10));

    explorer.fetch_merkle_root(tx_hash).await?;

    Ok(())
}

/*
https://testnet-api.smartbit.com.au/v1/blockchain/
https://api.bitaps.com/btc/testnet/native/
https://api.blockcypher.com/v1/btc/test3/
*/
