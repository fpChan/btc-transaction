use crate::btc_transaction::BTCNetwork;
use async_trait::async_trait;
use reqwest::Error;
use serde::Deserialize;
use std::{thread, time};

#[async_trait]
pub trait Explorer {
    async fn fetch_merkle_root(&self, tx_hash: String) -> Result<(), Error>;
}

#[async_trait]
pub trait BroadcastNode {
    async fn submit_tx(&self, tx_hex: String) -> Result<String, Error>;
}

#[derive(Deserialize, Debug)]
struct BlockcypherTxData {
    block_height: u128,
    hash: String,
}

#[derive(Deserialize, Debug)]
struct BlockcypherBlockData {
    mrkl_root: String,
    txids: Vec<String>,
}

pub struct Blockcypher {
    pub(crate) url: String,
}
#[async_trait]
impl Explorer for Blockcypher {
    async fn fetch_merkle_root(&self, tx_hash: String) -> Result<(), Error> {
        let tx: BlockcypherTxData = reqwest::get(&format!("{}txs/{}", self.url, tx_hash))
            .await?
            .json()
            .await?;

        let block: BlockcypherBlockData =
            reqwest::get(&format!("{}blocks/{}", self.url, tx.block_height))
                .await?
                .json()
                .await?;

        println!("mrkl_root: {:?}\n", block.mrkl_root);
        for txid in block.txids.iter() {
            println!("tx hash : {:?} ", txid)
        }
        Ok(())
    }
}

#[derive(Deserialize, Debug)]
struct BitapPostTxRespErr {
    code: i128,
    message: String,
}

#[derive(Deserialize, Debug)]
struct BitapPostTxResp {
    result: Option<String>,
    id: String,
    error: Option<BitapPostTxRespErr>,
}

#[derive(Deserialize, Debug)]
pub struct Bitaps {
    pub(crate) url: String,
}

#[async_trait]
impl BroadcastNode for Bitaps {
    async fn submit_tx(&self, tx_hex: String) -> Result<String, Error> {
        let params = format!(
            r#"{{"jsonrpc":"1.0", "id":"1", "method":"sendrawtransaction", "params":["{}"]}}"#,
            tx_hex
        );
        println!("params : {}", params);

        let response = reqwest::Client::new()
            .post(&format!("{}native/", self.url,))
            .body(params)
            .send()
            .await?;

        let result: BitapPostTxResp = response.json().await?;

        if !result.error.is_none() {
            println!("broadcast tx fail: {:?}", result.error.unwrap());
        }
        Ok((result.result.unwrap()))
    }
}

pub struct TestnetConf {
    pub(crate) submit_tx_url: String,
    pub(crate) query_tx_url: String,
}

impl Default for TestnetConf {
    fn default() -> Self {
        TestnetConf {
            submit_tx_url: "https://api.bitaps.com/btc/testnet/".to_string(),
            query_tx_url: "https://api.blockcypher.com/v1/btc/test3/".to_string(),
        }
    }
}
/*
https://testnet-api.smartbit.com.au/v1/blockchain/
https://api.bitaps.com/btc/testnet/native/
https://api.blockcypher.com/v1/btc/test3/
*/

#[async_trait]
impl BTCNetwork for TestnetConf {
    async fn broadcast_tx(&self, tx_hex: String) -> Result<(), Error> {
        let broadcast_node = Bitaps {
            url: self.submit_tx_url.clone(),
        };
        let explorer = Blockcypher {
            url: self.query_tx_url.clone(),
        };
        let tx_hash = broadcast_node.submit_tx(tx_hex).await?;

        thread::sleep(time::Duration::from_secs(60 * 10));

        explorer.fetch_merkle_root(tx_hash).await
    }
}
