use async_trait::async_trait;
use reqwest::Error;
use serde::Deserialize;

#[async_trait]
pub trait Explorer {
    async fn fetch_merkle_root(&self, tx_hash: String) -> Result<(), Error>;
}

#[async_trait]
pub trait BroadcastNode {
    async fn broadcast_tx(&self, tx_hex: String) -> Result<String, Error>;
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
    async fn broadcast_tx(&self, tx_hex: String) -> Result<String, Error> {
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

/*
#[async_trait]
impl BroadcastNode for Blockcypher {
    async fn broadcast_tx(&self, tx_hex: String) -> Result<(), Error> {
        let payload = format!("{{{:?}:{:?}}}", "hex", tx_hex);
        let v: Value = serde_json::from_str(payload.as_str()).unwrap();
        let client = reqwest::Client::new();

        let response = client
            .post(&format!("{}txs/push/", self.url))
            .json(&v)
            .send()
            .await?;

        let resp = response.json().await?;

        println!("{:?}", resp);

        Ok(())
    }
}
 */
