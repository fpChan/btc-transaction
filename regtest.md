## 一、部署私链节点
- 1、下载bitcoind and bitcoin-cli 
  ```shell 
    ➜   wget https://bitcoin.org/bin/bitcoin-core-0.20.0/bitcoin-0.20.0-x86_64-linux-gnu.tar.gz
    ➜   tar -xjvf bitcoin-0.20.0-x86_64-linux-gnu.tar.gz -C ./
    ➜   export PATH=~/work/btc/bitcoin-0.20.0/bin:$PATH
  ```


- 2、配置文件, 编辑 `/etc/bitcoin/bitcoin.conf`

  ```
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

  

- 3、启动本地私链

  ```shell
  ➜ bitcoind -conf=/etc/bitcoin/bitcoin.conf
  Bitcoin Core starting
  ```

  

- 4、确认`bitcoind` 监听的端口,  本地一般是 18843

  ```shell
  ➜  netstat --ip -lpa | grep bitcoind
  tcp   0  0   localhost:18443     0.0.0.0:*       LISTEN      6768/bitcoind
  ```

  

- 5、发送请求, 测试服务是否运行

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

  

- 6、导入私钥进钱包, 默认钱包名称为 ""

  ```shell
  ➜ bitcoin-cli -conf=/etc/bitcoin/bitcoin.conf  -rpcwallet="" importprivkey "cUDfdzioB3SqjbN9vutRTUrpw5EH9srrg6RPibacPo1fGHpfPKqL"
  ```

  

- 7、构造生成块,  需要生成101个块, 如果需要使用coinbase交易, 需要100个块后确认, 最好多于100个块

  ```shell
  ➜  bitcoin-cli -conf=/etc/bitcoin/bitcoin.conf generatetoaddress 101 mvuTXVzk7n9QYHMhyUUfaPdeQ4QVwA2fmT
  ```

  

- 8、根据hash `2d9ce904ac4d0df8dffec9a9a7e2c65883c357c37bceab044fb70f3515f9720c`查询块内交易

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

  

- 9、根据交易hash `d0bce39e8a8a52ec58e71cb0acc28f21a2d54917e9c201619454d65078766ce4` 查询交易

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





## 二、构建运行，发送交易到私链

- 1、修改代码中的配置(尚未改为文件读取配置)

   btc_transaction 中build_tx_hex 私钥，脚本类型，send_amount,total_amount都需要根据input进行更改
  ```toml
  # tx_hash + index 作为 交易输入
  out_point= "a8d7ecefd3f8c60d646a246498906f78d886bf3ceec6bc1daa595b70ddde7989:0"
  
  # out_point 中的total_amount 需要作为参数传递(P2WPKH签名需要)
  
  # from 的私钥必须要，to的私钥可以不需要（只需要提供脚本就可）
  
  ```

  

- 2、运行, 线程执行sleep时，执行`bitcoin-cli -conf=/etc/bitcoin/bitcoin.conf generatetoaddress 1 mvuTXVzk7n9QYHMhyUUfaPdeQ4QVwA2fmT` （出块打包该交易）

  ```shell
  ➜  btc_tx git:(master) ✗ ~/proxyrc.sh cargo run
     Compiling btc_transaction v0.1.0 (/mnt/d/Work/Rust/demo/btc_tx)
      Finished dev [unoptimized + debuginfo] target(s) in 29.46s
       Running `target/debug/btc_transaction`
       
  tx hash: bf0ad48ab9b4d44ece0640f7436fe4214170e3af4392c3818a709a0a50796794
  
  encode tx :010000000199b6f6f9358cf94e3ab6050b40652238d27aa353868bcea59d5abb9202d711ac000000006a473044022035a93d4fb71323df30840b9e71068bc18c7e316d9c95a021b01c7dca5bbabb1c022076d5fb1b84d3fcf859db96abd7e384b4bade179d02630dbb349c376378211a3a01210227de674775b35b06fca8ed06a492c817d542cc08b8d4f64d3717d4af70134d8000000000016072019500000000160014489b80d23c5148b2f29c8b5f03478a4d0dd67a0000000000
  tx hash: bf0ad48ab9b4d44ece0640f7436fe4214170e3af4392c3818a709a0a50796794
  
  check_merkle_root :true
  
  merkle_root : 9a3cb7f9e3ff1b434a38bd267e9c25a95ed4ee461b761252ed38bd9f75f35975
  
  best proof : 00000030ca889853e7337822f2673f6153b92f4ba376ff823b5b2d262dc2c49e60cd24717559f3759fbd38ed5212761b46eed45ea9259c7e26bd384a431bffe3f9b73c9a73a6695fffff7f20000000000300000002f707925e6d07fc7dc7ec5967091c69f227c76b19b55319d4ffdd17107b71b5c6946779500a9a708a81c39243afe3704121e46f43f74006ce4ed4b4b98ad40abf010d
  ```
  
- 3、验证，查询交易`bf0ad48ab9b4d44ece0640f7436fe4214170e3af4392c3818a709a0a50796794` 的output
  ```shell script
    ➜  bitcoin-cli -conf=/etc/bitcoin/bitcoin.conf generatetoaddress 1 mvuTXVzk7n9QYHMhyUUfaPdeQ4QVwA2fmT
    [
      "402b8f550a1cbeb112e834f04092a68abe5972238da23074c54eec87353b78d9"
    ]
    ➜  bitcoin-cli -conf=/etc/bitcoin/bitcoin.conf gettxout bf0ad48ab9b4d44ece0640f7436fe4214170e3af4392c3818a709a0a50796794 0
    {
      "bestblock": "402b8f550a1cbeb112e834f04092a68abe5972238da23074c54eec87353b78d9",
      "confirmations": 1,
      "value": 24.99900000,
      "scriptPubKey": {
        "asm": "0 489b80d23c5148b2f29c8b5f03478a4d0dd67a00",
        "hex": "0014489b80d23c5148b2f29c8b5f03478a4d0dd67a00",
        "reqSigs": 1,
        "type": "witness_v0_keyhash", # output 为 P2WPKH 脚本
        "addresses": [
          "bcrt1qfzdcp53u29yt9u5u3d0sx3u2f5xav7sqatfxm2"
        ]
      },
      "coinbase": false
    }
   ```

  
  ## 参考文档
  
[bitcoin 0.20 rpc文档](https://dg0.dtrt.org/en/doc/0.20.0/rpc/blockchain/getblock/)