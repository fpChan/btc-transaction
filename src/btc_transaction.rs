use async_trait::async_trait;
use bitcoin::blockdata::script::Builder;
use bitcoin::hashes::core::str::FromStr;
use bitcoin::hashes::Hash;
use bitcoin::secp256k1::{All, Message, Secp256k1};
use bitcoin::util::psbt::serialize::Serialize;
use bitcoin::{Address, Network, OutPoint, PublicKey, SigHashType, Transaction, TxIn, TxOut};
use bitcoin::{PrivateKey, Script};
use hex::encode;
use reqwest::Error;

#[derive(Debug)]
pub struct Account {
    pub private_key: PrivateKey,
    pub public_key: PublicKey,
    pub p2pwkh_addr: Address,
    pub p2pkh_addr: Address,
}

fn load_account(privkey: &str) -> Account {
    let sk = PrivateKey::from_str(privkey).unwrap();
    let secp: Secp256k1<All> = Secp256k1::new();
    let pk = sk.public_key(&secp);

    let p2pwkh_address = Address::p2wpkh(&pk, Network::Testnet).unwrap();
    let p2pkh_address = Address::p2pkh(&pk, Network::Testnet);

    Account {
        private_key: sk,
        public_key: pk,
        p2pwkh_addr: p2pwkh_address,
        p2pkh_addr: p2pkh_address,
    }
}

pub enum TxType {
    P2PKH,
    P2WPKH,
}

pub fn build_tx_hex(
    out_point: &str,
    send_amount: u64,
    total_amount: u64,
    tx_type: TxType,
) -> String {
    let from_privkey = "cUDfdzioB3SqjbN9vutRTUrpw5EH9srrg6RPibacPo1fGHpfPKqL";
    let to_privkey = "cU9PYTnSkcWoAE15U26JJCwtKiYvTCKYdbWt8e7ovidEGDBwJQ5x";
    let to_acc = load_account(to_privkey);

    match tx_type {
        TxType::P2PKH => build_spend_p2pkh_tx(
            from_privkey,
            out_point,
            send_amount,
            to_acc.p2pwkh_addr.script_pubkey(),
        ),
        TxType::P2WPKH => build_spend_p2pwkh_tx(
            from_privkey,
            out_point,
            send_amount,
            total_amount,
            to_acc.p2pkh_addr.script_pubkey(),
        ),
    }
}

pub fn build_spend_p2pkh_tx(
    from_privkey: &str,
    out_point: &str,
    send_amount: u64,
    output_script: Script,
) -> String {
    let from_acc = load_account(from_privkey);

    let mut raw_tx = Transaction {
        version: 1,
        lock_time: 0,
        input: vec![TxIn {
            previous_output: OutPoint::from_str(out_point).unwrap(),
            script_sig: Script::new(),
            sequence: 0,
            witness: vec![],
        }],
        output: vec![TxOut {
            value: send_amount,
            script_pubkey: output_script,
        }],
    };
    let sig_hash = raw_tx.signature_hash(0, &from_acc.p2pkh_addr.script_pubkey(), 0x1);

    let msg = Message::from_slice(&sig_hash.into_inner()).unwrap();
    let secp: Secp256k1<All> = Secp256k1::new();
    let sig = secp.sign(&msg, &from_acc.private_key.key).serialize_der();
    let mut sigser = sig.to_vec();
    sigser.push(0x01); // sighash_all

    let script = Builder::new()
        .push_slice(&sigser[..])
        .push_key(&from_acc.public_key)
        .into_script();

    raw_tx.input[0].script_sig = script;
    println!("tx hash: {:?} \n", raw_tx.txid());

    encode(raw_tx.serialize())
}

pub fn build_spend_p2pwkh_tx(
    from_privkey: &str,
    out_point: &str,
    send_amount: u64,
    total_amount: u64,
    output_script: Script,
) -> String {
    let from_acc = load_account(from_privkey);

    let mut raw_tx = Transaction {
        version: 1,
        lock_time: 0,
        input: vec![TxIn {
            previous_output: OutPoint::from_str(out_point).unwrap(),
            script_sig: Script::new(),
            sequence: 0,
            witness: vec![],
        }],
        output: vec![TxOut {
            value: send_amount,
            script_pubkey: output_script,
        }],
    };

    let mut sighash_components = bitcoin::util::bip143::SigHashCache::new(&raw_tx);
    let sig_hash = sighash_components.signature_hash(
        0,
        &from_acc.p2pkh_addr.script_pubkey(),
        total_amount,
        SigHashType::All,
    );
    let msg = Message::from_slice(&sig_hash.into_inner()).unwrap();

    let secp: Secp256k1<All> = Secp256k1::new();
    let mut sigser = secp
        .sign(&msg, &from_acc.private_key.key)
        .serialize_der()
        .to_vec();

    sigser.push(0x01);
    raw_tx.input[0].witness = vec![sigser, from_acc.public_key.to_bytes()];
    println!("tx hash: {:?} \n", raw_tx.txid());

    encode(raw_tx.serialize())
}

#[async_trait]
pub trait BTCNetwork {
    async fn broadcast_tx(&self, tx_hex: String) -> Result<(), Error>;
}
