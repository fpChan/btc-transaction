## 部署私链, 获得有效 input

- 1、配置文件, 编辑 `/etc/bitcoin/bitcoin.conf`

  ```toml
  daemon=1
  server=1
  rpcuser=test
  rpcpassword=test
  regtest=1
  txindex=1
  rpcallowip=0.0.0.0/0
  discover=0
  listen=0
  ```

  

- 2、启动本地私链

  ```shell
  ➜ bitcoind -conf=/etc/bitcoin/bitcoin.conf
  Bitcoin Core starting
  ```

  

- 3、获取`bitcoind` 监听的端口,  本地一般是 18843

  ```shell
  ➜  netstat --ip -lpa | grep bitcoind
  tcp   0  0   localhost:18443     0.0.0.0:*       LISTEN      6768/bitcoind
  ```

  

- 4、发送请求, 测试服务是否运行

  ```shell
  ➜  curl --request POST \
      --user test:test \
      --data-binary '{"jsonrpc": "1.0", "id":"curltest", "method": "getblockchaininfo", "params": [] }' \
      -H 'content-type: applicaiton/json;' \
      http://127.0.0.1:18443/
  {
  	"result": {
  		"chain": "regtest",
  		"blocks": 303,
  		"headers": 303,
  		"bestblockhash": "5eba253aed009e25d347d59906c39bf47ae13ca7dc6ccd58c41edd549dd06214",
  		"difficulty": 4.656542373906925e-10,
  		"mediantime": 1600689629,
  		"verificationprogress": 1,
  		"initialblockdownload": false,
  		"chainwork": "0000000000000000000000000000000000000000000000000000000000000260",
  		"size_on_disk": 91959,
  		"pruned": false,
  		"softforks": {
  			.......
  		},
  		"warnings": ""
  	},
  	"error": null,
  	"id": "curltest"
  }
  ```

  

- 5、导入私钥进钱包, 默认钱包名称为 ""

  ```shell
  ➜ bitcoin-cli -conf=/etc/bitcoin/bitcoin.conf  -rpcwallet="" importprivkey "cUDfdzioB3SqjbN9vutRTUrpw5EH9srrg6RPibacPo1fGHpfPKqL"
  ```

  

- 6、构造生成块,  需要生成101个块, 如果需要使用coinbase交易, 需要100个块后确认, 最好多于100个块

  ```shell
  ➜  bitcoin-cli -conf=/etc/bitcoin/bitcoin.conf generatetoaddress 101 mvuTXVzk7n9QYHMhyUUfaPdeQ4QVwA2fmT
  ```

  

- 7、根据hash `2d9ce904ac4d0df8dffec9a9a7e2c65883c357c37bceab044fb70f3515f9720c`查询块内交易

  ```shell
  ➜  bitcoin-cli -conf=/etc/bitcoin/bitcoin.conf getblock 2d9ce904ac4d0df8dffec9a9a7e2c65883c357c37bceab044fb70f3515f9720c
  {
    "hash": "2d9ce904ac4d0df8dffec9a9a7e2c65883c357c37bceab044fb70f3515f9720c",
    "confirmations": 196,
    "strippedsize": 217,
    "size": 253,
    "weight": 904,
    "height": 108,
    "version": 536870912,
    "versionHex": "20000000",
    "merkleroot": "d0bce39e8a8a52ec58e71cb0acc28f21a2d54917e9c201619454d65078766ce4",
    "tx": [
      "d0bce39e8a8a52ec58e71cb0acc28f21a2d54917e9c201619454d65078766ce4"
    ],
    "time": 1600689597,
    "mediantime": 1600689596,
    "nonce": 0,
    "bits": "207fffff",
    "difficulty": 4.656542373906925e-10,
    "chainwork": "00000000000000000000000000000000000000000000000000000000000000da",
    "nTx": 1,
    "previousblockhash": "642bf8c9135ef0fc37799a7348ecbc5d4e70a89cfce7e59586ad6fe34b64e093",
    "nextblockhash": "1f767a69b1c168ba55ade509c2b5cca11d848672e331a91dc5035778b243c0a7"
  }
  
  ```

  

- 8、根据交易hash `d0bce39e8a8a52ec58e71cb0acc28f21a2d54917e9c201619454d65078766ce4` 查询交易

  ```shell
  ➜  bitcoin-cli -conf=/etc/bitcoin/bitcoin.conf gettxout d0bce39e8a8a52ec58e71cb0acc28f21a2d54917e9c201619454d65078766ce4 0
  {
    "bestblock": "5eba253aed009e25d347d59906c39bf47ae13ca7dc6ccd58c41edd549dd06214",
    "confirmations": 196,#coinbase 交易需要100个确认才能使用
    "value": 50.00000000,
    "scriptPubKey": {
      "asm": "OP_DUP OP_HASH160 a8cb707e4d0a5c6e690189bc0065a8f787aabced OP_EQUALVERIFY OP_CHECKSIG",
      "hex": "76a914a8cb707e4d0a5c6e690189bc0065a8f787aabced88ac",
      "reqSigs": 1,
      "type": "pubkeyhash",
      "addresses": [
        "mvuTXVzk7n9QYHMhyUUfaPdeQ4QVwA2fmT"
      ]
    },
    "coinbase": true
  }
  ```





## 运行代码, 往私链发送交易

- 修改代码中的配置(尚未改为文件读取配置)

  ```toml
  # tx_hash + index 作为 交易输入
  out_point= "a8d7ecefd3f8c60d646a246498906f78d886bf3ceec6bc1daa595b70ddde7989:0";
  
  # out_point 中的total_value 需要作为参数传递(P2WPKH签名需要)
  
  ```

  

- 运行

  ```shell
  ➜  btc_tx git:(master) ✗ ~/proxyrc.sh cargo run
     Compiling btc_transaction v0.1.0 (/mnt/d/Work/Rust/demo/btc_tx)
      Finished dev [unoptimized + debuginfo] target(s) in 29.46s
       Running `target/debug/btc_transaction`
       
  tx hash: 44f27e38afbcf30c7559159290f754761852e0a654852aba1fbf81ca9a4f7279
  
  encode tx :01000000018979dedd705b59aa1dbcc6ee3cbf86d8786f909864246a640dc6f8d3efecd7a8000000006a47304402205b8205b309e6b5ebee8c8979c0f60ee82926f4389a33f3a9f40e479eb561b1f602207e46f79ce0e93b9b6b871424c9ca002f35c9351352214f47ffdf53c97250e0ca01210227de674775b35b06fca8ed06a492c817d542cc08b8d4f64d3717d4af70134d800000000001606b042a01000000160014489b80d23c5148b2f29c8b5f03478a4d0dd67a0000000000
  
  tx hash: 44f27e38afbcf30c7559159290f754761852e0a654852aba1fbf81ca9a4f7279
  
  check_merkle_root :true
  
  merkle_root : f078fd2f40a13f228f937add80bd996aa2a69e389d386843f8355fcdbf77da7a
  
  best proof : 00000030ca889853e7337822f2673f6153b92f4ba376ff823b5b2d262dc2c49e60cd24717ada77bfcd5f35f84368389d389ea6a26a99bd80dd7a938f223fa1402ffd78f0e2ca685fffff7f2002000000020000000205bdb64475a0efd9d05c6d25ff44a69959579edb08d71cbf745baee3ce634ead79724f9aca81bf1fba2a8554a6e052187654f790921559750cf3bcaf387ef2440105
  ```

  