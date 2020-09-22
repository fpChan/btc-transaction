use crate::btc_transaction::BTCNetwork;
use async_trait::async_trait;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use hex::encode;
use reqwest::Error;
use std::{thread, time};

pub struct RegtestConf {
    pub(crate) url: String,
    pub(crate) user: String,
    pub(crate) password: String,
}

impl Default for RegtestConf {
    fn default() -> Self {
        RegtestConf {
            url: "http://localhost:18443".to_string(),
            user: "test".to_string(),
            password: "test".to_string(),
        }
    }
}

#[async_trait]
impl BTCNetwork for RegtestConf {
    async fn broadcast_tx(&self, tx_hex: String) -> Result<(), Error> {
        let rpc = Client::new(
            self.url.clone(),
            Auth::UserPass(self.user.clone(), self.password.clone()),
        )
        .unwrap();

        let txid = rpc.send_raw_transaction(tx_hex).unwrap();
        println!(" tx hash: {} \n", txid);

        // execute cmd ` bitcoin-cli -conf=/etc/bitcoin/bitcoin.conf generatetoaddress 1 mvuTXVzk7n9QYHMhyUUfaPdeQ4QVwA2fmT`  in 60 secs.
        thread::sleep(time::Duration::from_secs(60 * 1));

        let txs = vec![txid];
        let tx_result = rpc.get_transaction(&txid, None).unwrap();
        let block_hash = tx_result.info.blockhash.unwrap();
        let block = rpc.get_block(&block_hash).unwrap();
        println!("check_merkle_root :{}\n", block.check_merkle_root());
        println!("merkle_root : {}\n", block.header.merkle_root.to_string());
        let proof = rpc
            .get_tx_out_proof(&*txs, Option::from(&block_hash))
            .unwrap();
        println!("best proof : {}\n", encode(proof));
        Ok(())
    }
}
