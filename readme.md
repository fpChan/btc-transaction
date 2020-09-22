## 一、 需求背景

在实现跨链转账的`demo `中，当`BTC`链收到交易信号时，需要从`BTC`链 地址A 往 地址B 发送`btc`，获得 `merkle root` 进行以便后续验证。需要完成这部分功能



## 二、大致思路

目前在 `Bitcoin Testnet ` 实现， 发送 `P2PKH` 交易、 `P2WPKH` 交易

从水龙头获取`BTC` 测试币，通过现有的一些 [`btc rust lib`](https://docs.rs/bitcoin/0.25.0/bitcoin/index.html)   构造交易，签名， 发送交易到一些全节点 (自行部署测试网节点 成本较高)



## 三、 prepare

### 1、 account

  ```yaml
  # first 
  privkey:  cUDfdzioB3SqjbN9vutRTUrpw5EH9srrg6RPibacPo1fGHpfPKqL
  address:  mvuTXVzk7n9QYHMhyUUfaPdeQ4QVwA2fmT
  mnemonic: banner citizen mimic such symptom ivory wolf flower pear female deputy surge
  
  # second
  privkey:  cU9PYTnSkcWoAE15U26JJCwtKiYvTCKYdbWt8e7ovidEGDBwJQ5x
  address:  mn8sERCo1gTrAt1jfbUBN4bXXS67ykESho
  mnemonic: lazy hub spring employ rent exit differ tongue feature badge stage extend
  ```

### 2、rust library

  - https://docs.rs/bitcoin/0.25.0/bitcoin/index.html
  - https://github.com/rust-bitcoin/rust-bitcoincore-rpc

### 3、Network

#### 3.1、 Testnet

 - faucet

     - https://testnet-faucet.mempool.co/

     - https://bitcoinfaucet.uo1.net/

- broadcast tx api

  - https://tbtc.bitaps.com/broadcast

    ```http
    POST  https://api.bitaps.com/btc/testnet/native/
    
    // params 
    {"jsonrpc":"1.0","id":"1","method":"testmempoolaccept","params":[["010000000001013a135af747f5628d1ba65fda2eea99a3e570ea047e77740402ea51479b476ff4000000000000000000010833010000000000160014a8cb707e4d0a5c6e690189bc0065a8f787aabced024830450221009c6e1af5a9c0d1fa942a462fc2f5dbb5971a4112d8f425a46822e7c38a15cfa20220173b83f0f577ceb3bea02f09f04159132e25886c5c52bd166192a1d75f1f7ba801210227de674775b35b06fca8ed06a492c817d542cc08b8d4f64d3717d4af70134d8000000000"]]}
    ```
    
#### 3.2、 Regtest

部署以及运行可见[regtest.md](./regtest.md)



## 四、详细内容


### 代码实现

构造交易，签名，获取`raw_transaction ` ， 然后将数据发送到浏览器站点

####  `P2PKH` 

将 `P2PKH` 脚本作为 input，将 `P2WPKH`  作为 output

构造（签名 + 公钥）  作为解锁脚本

####  `P2WPKH` 

将 `P2WPKH` 脚本作为 input，将 `P2PKH`  作为 output

构造（签名 + 公钥）作为 [witness](https://github.com/fpChan/btc-transaction/blob/5c83e5a705f55155867316978453a6bff98999a0/src/btc_transaction.rs#L142)
