# 1 build taptool for test env

```bash
git clone https://github.com/bitcoin-core/btcdeb.git
export ENABLE_DANGEROUS=1
./autogen.sh
./configure --enable-dangerous
make
sudo make install
```

# 2 build and run bitcoin network

## in linux

```bash
git clone https://github.com/bitcoin/bitcoin.git
cd bitcoin
git checkout a0988140b7
./autogen.sh
./configure CXXFLAGS='-O0 -g' --with-gui=yes  --enable-module-schnorrsig  
make
sudo make install

bitcoin-qt  -regtest -server -rpcuser=rpcuser -rpcpassword=rpcpassword -rpcport=8332 -fallbackfee=0.000001 -txindex
```

## Also, you can also download and run. RECOMMENDED FOR MAC AND NIXOS

[Download executables](https://bitcoin.org/en/download)

```bash
/Applications/Bitcoin-Qt.app/Contents/MacOS/Bitcoin-Qt -regtest -server -rpcuser=rpcuser -rpcpassword=rpcpassword -rpcport=18332 -fallbackfee=0.000001 -txindex
```

## Generate some funds

* Click receive on UI
* Generate address and copy it
* Go to Window -> Console
* type: `generatetoaddress 101 {Address}`
* FYI: 101 block generated to make available at least one block reward for spending.

# 3 build locktime tap script

## init key and script

Exporting env variables:

```bash
export privkey=1229101a0fcf2104e8808dab35661134aa5903867d44deb73ce1c7e4eb925be8
export pubkey=f30544d6009c8d8d94f5d030b2e844b1a3ca036255161c479db1cca5b374dd1c

export script_alice='[100 OP_CHECKLOCKTIMEVERIFY OP_DROP 9997a497d964fc1a62885b05a51166a65a90df00492c8d7cf61d6accf54803be OP_CHECKSIG]'

export script_bob='[4edfcf9dfe6c0b5c83d1ab3f78d1b39a46ebac6798e08e19761f5ed89ec83c10 OP_CHECKSIG OP_FALSE OP_IF OP_3 6f7264 OP_1 1 0x1e 6170706c69636174696f6e2f6a736f6e3b636861727365743d7574662d38 OP_1 5 0x4b   7b73656e6465723a20223465646663663964666536633062356338336431616233663738643162333961343665626163363739386530386531393736316635656438396563383363313022 OP_ENDIF]'

```

## product script Bech32m address

Executing:

```bash
tap $pubkey 2 "${script_alice}" "${script_bob}" 
```

Expected output:

```bash
tap 5.0.24 -- type `tap -h` for help
WARNING: This is experimental software. Do not use this with real bitcoin, or you will most likely lose them all. You have been w a r n e d.
LOG: sign segwit taproot
Internal pubkey: f30544d6009c8d8d94f5d030b2e844b1a3ca036255161c479db1cca5b374dd1c
2 scripts:
- #0: 0164b175209997a497d964fc1a62885b05a51166a65a90df00492c8d7cf61d6accf54803beac
warning: ambiguous input 1 is interpreted as a numeric value; use OP_1 to force into opcode
warning: ambiguous input 5 is interpreted as a numeric value; use OP_5 to force into opcode
- #1: 204edfcf9dfe6c0b5c83d1ab3f78d1b39a46ebac6798e08e19761f5ed89ec83c10ac006353036f72645151011e1e6170706c69636174696f6e2f6a736f6e3b636861727365743d7574662d385155014b4b7b73656e6465723a2022346564666366396466653663306235633833643161623366373864316233396134366562616336373938653038653139373631663565643839656338336331302268
Script #0 leaf hash = TapLeaf<<0xc0 || 0164b175209997a497d964fc1a62885b05a51166a65a90df00492c8d7cf61d6accf54803beac>>
 → 0ce303b5acc6a0e96a3cdf926242938f6c238b16c04c2a428b44b2634ca6613c
Script #1 leaf hash = TapLeaf<<0xc0 || 204edfcf9dfe6c0b5c83d1ab3f78d1b39a46ebac6798e08e19761f5ed89ec83c10ac006353036f72645151011e1e6170706c69636174696f6e2f6a736f6e3b636861727365743d7574662d385155014b4b7b73656e6465723a2022346564666366396466653663306235633833643161623366373864316233396134366562616336373938653038653139373631663565643839656338336331302268>>
 → 318d714b33394c4281b188ea3d3350d92011c8599550bae06813dbcec033c33b
Branch (#0, #1)
 → df0a2fb8098e18bd5083ae8acb41e20aae02daf838245013fa8d2f42cd5cf78d
Tweak value = TapTweak(f30544d6009c8d8d94f5d030b2e844b1a3ca036255161c479db1cca5b374dd1c || df0a2fb8098e18bd5083ae8acb41e20aae02daf838245013fa8d2f42cd5cf78d) = c9aac2f9b79c96377c9aa3d66bdef93a3bd86fa68cfbf05128bceb2db0fde869
Tweaked pubkey = c9c115285680e6dabffd671e807291497aaa5109edfc7437814679c2aa429bc9 (even)
Resulting Bech32m address: bcrt1pe8q322zksrnd40lavu0gqu53f9a255gfah78gdupgeuu92jzn0ysnry0c9
```

# 4 send token to script bech32m address

You can do the same on the console ui

```bash
sendtoaddress bcrt1pe8q322zksrnd40lavu0gqu53f9a255gfah78gdupgeuu92jzn0ysnry0c9 0.001
```

or using CLI

```bash
bcli -rpcuser=rpcuser -rpcpassword=rpcpassword sendtoaddress bcrt1pe8q322zksrnd40lavu0gqu53f9a255gfah78gdupgeuu92jzn0ysnry0c9 0.001
```

## Output

We expect to see some transaction hash:

```bash
e18be5221d15aee6dc3918d7c63362a98ecc04741c8a78106aec2625f5286bb2
```

Let's save the transaction hash:

```bash
export tx_hash="e18be5221d15aee6dc3918d7c63362a98ecc04741c8a78106aec2625f5286bb2"
```

# 5 get tx detail

CLI:

```bash
bcli -rpcuser=rpcuser -rpcpassword=rpcpassword getrawtransaction $tx_hash 1
```

UI:

```bash
getrawtransaction e18be5221d15aee6dc3918d7c63362a98ecc04741c8a78106aec2625f5286bb2 1
```

## Output

> Please look for vout with the `bcrt1pe8q322zksrnd40lavu0gqu53f9a255gfah78gdupgeuu92jzn0ysnry0c9` address. And look for the "n" param. We would need to specify it later.

```json
{
  "txid": "e18be5221d15aee6dc3918d7c63362a98ecc04741c8a78106aec2625f5286bb2",
  "hash": "09c87c6cff6312cb7a50a34bd7ccf0fdc7ccb7782d8eca0204dab9384940c694",
  "version": 2,
  "size": 542,
  "vsize": 300,
  "weight": 1199,
  "locktime": 332,
  "vin": [
    {
      "txid": "b058e1ad4b784cbc8c0bc29eed425d9874bd1d4781b909ba9c8bd5ea0b094c76",
      "vout": 0,
      "scriptSig": {
        "asm": "",
        "hex": ""
      },
      "txinwitness": [
        "304402205761fff2a1581265c3284e7717639cab12649deff116db499fb43cd0f879a791022023ce08e3ec920c209cdaefecc578ce90d5000bd582b0a8ca835e597355059b1c01",
        "02d8911002f179a15722867b9c3893084a8f59fc73608f05b5f2b76fee65f672fb"
      ],
      "sequence": 4294967293
    },
    {
      "txid": "76f3b95d05e7daec697c4a81eba09a33888dee9bc3739a72ce79f5253d13d475",
      "vout": 0,
      "scriptSig": {
        "asm": "",
        "hex": ""
      },
      "txinwitness": [
        "304402205b7ade74f59814a9d34d2fe72c57adf7a0b4ad0547671cd2fc549acfb58b04ca02205f7e0cbe47e024a2773a5354193c6b93bc81482c28f2ca3e501f7b5ff50c99f301",
        "02d8911002f179a15722867b9c3893084a8f59fc73608f05b5f2b76fee65f672fb"
      ],
      "sequence": 4294967293
    },
    {
      "txid": "7c33483f32de56334252df53ec6cf9944f3ef710490ce44b921ba2ea28eec583",
      "vout": 0,
      "scriptSig": {
        "asm": "",
        "hex": ""
      },
      "txinwitness": [
        "304402201de1546e65f5ba97e3b7433dccfdb82003fc9a38004c853b8aaa3547d171f9a802205f8e2dc6ca1d10ba2e12304a308f62dee7632a2a5834b1069083a36139189eae01",
        "02d8911002f179a15722867b9c3893084a8f59fc73608f05b5f2b76fee65f672fb"
      ],
      "sequence": 4294967293
    }
  ],
  "vout": [
    {
      "value": 0.00100000,
      "n": 0,
      "scriptPubKey": {
        "asm": "1 c9c115285680e6dabffd671e807291497aaa5109edfc7437814679c2aa429bc9",
        "desc": "rawtr(c9c115285680e6dabffd671e807291497aaa5109edfc7437814679c2aa429bc9)#7j3lu034",
        "hex": "5120c9c115285680e6dabffd671e807291497aaa5109edfc7437814679c2aa429bc9",
        "address": "bcrt1pe8q322zksrnd40lavu0gqu53f9a255gfah78gdupgeuu92jzn0ysnry0c9",
        "type": "witness_v1_taproot"
      }
    },
    {
      "value": 0.00169700,
      "n": 1,
      "scriptPubKey": {
        "asm": "1 86b974d6cafc8ca1641b204018a1fac81535742b90bc5bae7c3d5be21fb1cd87",
        "desc": "rawtr(86b974d6cafc8ca1641b204018a1fac81535742b90bc5bae7c3d5be21fb1cd87)#pxszdks7",
        "hex": "512086b974d6cafc8ca1641b204018a1fac81535742b90bc5bae7c3d5be21fb1cd87",
        "address": "bcrt1ps6uhf4k2ljx2zeqmypqp3g06eq2n2aptjz79htnu84d7y8a3ekrs5kjjy2",
        "type": "witness_v1_taproot"
      }
    }
  ],
  "hex": "02000000000103764c090bead58b9cba09b981471dbd74985d42ed9ec20b8cbc4c784bade158b00000000000fdffffff75d4133d25f579ce729a73c39bee8d88339aa0eb814a7c69ecdae7055db9f3760000000000fdffffff83c5ee28eaa21b924be40c4910f73e4f94f96cec53df52423356de323f48337c0000000000fdffffff02a086010000000000225120c9c115285680e6dabffd671e807291497aaa5109edfc7437814679c2aa429bc9e49602000000000022512086b974d6cafc8ca1641b204018a1fac81535742b90bc5bae7c3d5be21fb1cd870247304402205761fff2a1581265c3284e7717639cab12649deff116db499fb43cd0f879a791022023ce08e3ec920c209cdaefecc578ce90d5000bd582b0a8ca835e597355059b1c012102d8911002f179a15722867b9c3893084a8f59fc73608f05b5f2b76fee65f672fb0247304402205b7ade74f59814a9d34d2fe72c57adf7a0b4ad0547671cd2fc549acfb58b04ca02205f7e0cbe47e024a2773a5354193c6b93bc81482c28f2ca3e501f7b5ff50c99f3012102d8911002f179a15722867b9c3893084a8f59fc73608f05b5f2b76fee65f672fb0247304402201de1546e65f5ba97e3b7433dccfdb82003fc9a38004c853b8aaa3547d171f9a802205f8e2dc6ca1d10ba2e12304a308f62dee7632a2a5834b1069083a36139189eae012102d8911002f179a15722867b9c3893084a8f59fc73608f05b5f2b76fee65f672fb4c010000"
}
```

# 6 create Taproot spend

Use your own hex output and fill txin and txid as showed below.

```bash
# In our case, bcrt1pe8q322zksrnd40lavu0gqu53f9a255gfah78gdupgeuu92jzn0ysnry0c9 vout has n = 0, you can have other value
export vout=0

# "hex" field
export txin='02000000000103764c090bead58b9cba09b981471dbd74985d42ed9ec20b8cbc4c784bade158b00000000000fdffffff75d4133d25f579ce729a73c39bee8d88339aa0eb814a7c69ecdae7055db9f3760000000000fdffffff83c5ee28eaa21b924be40c4910f73e4f94f96cec53df52423356de323f48337c0000000000fdffffff02a086010000000000225120c9c115285680e6dabffd671e807291497aaa5109edfc7437814679c2aa429bc9e49602000000000022512086b974d6cafc8ca1641b204018a1fac81535742b90bc5bae7c3d5be21fb1cd870247304402205761fff2a1581265c3284e7717639cab12649deff116db499fb43cd0f879a791022023ce08e3ec920c209cdaefecc578ce90d5000bd582b0a8ca835e597355059b1c012102d8911002f179a15722867b9c3893084a8f59fc73608f05b5f2b76fee65f672fb0247304402205b7ade74f59814a9d34d2fe72c57adf7a0b4ad0547671cd2fc549acfb58b04ca02205f7e0cbe47e024a2773a5354193c6b93bc81482c28f2ca3e501f7b5ff50c99f3012102d8911002f179a15722867b9c3893084a8f59fc73608f05b5f2b76fee65f672fb0247304402201de1546e65f5ba97e3b7433dccfdb82003fc9a38004c853b8aaa3547d171f9a802205f8e2dc6ca1d10ba2e12304a308f62dee7632a2a5834b1069083a36139189eae012102d8911002f179a15722867b9c3893084a8f59fc73608f05b5f2b76fee65f672fb4c010000'

# Take a look in the transaction info about vout of the bcrt1ps6uhf4k2ljx2zeqmypqp3g06eq2n2aptjz79htnu84d7y8a3ekrs5kjjy2 address. You need to look for `n` argument. It is either 1 or 0.

export tx=$(bcli -rpcuser=rpcuser -rpcpassword=rpcpassword createrawtransaction "[{\"txid\":\"$tx_hash\", \"vout\":$vout}]" '[{"bcrt1qg4xrdyf0dzc26y39zyzkajleww5z0hgzvzl9fj":0.0009}]') 

```

tips: you can create locktime spend tx with below, 100 is the locktime param.

```bash
tx=$(bcli -rpcuser=rpcuser -rpcpassword=rpcpassword  createrawtransaction "[{\"txid\":\"$tx_hash\", \"vout\":$vout}]" '[{"bcrt1qg4xrdyf0dzc26y39zyzkajleww5z0hgzvzl9fj":0.0009}]') 100

```

## Expect to see hash

`echo $tx`:

```
0200000001b26b28f52526ec6a10788a1c7404cc8ea96233c6d71839dce6ae151d22e58be10000000000fdffffff01905f010000000000160014454c36912f68b0ad122511056ecbf973a827dd0200000000
```

## Preparing transactions

```bash
tap -k81b637d8fcd2c6da6359e6963113a1170de795e4b725b84d1e0b4cfd9ec58ce9 --tx=$tx --txin=$txin $pubkey 2 "${script_alice}" "${script_bob}"  1
```

Expected output:

```bash
tap 5.0.24 -- type `tap -h` for help
WARNING: This is experimental software. Do not use this with real bitcoin, or you will most likely lose them all. You have been w a r n e d.
LOG: sign segwit taproot
targeting transaction vin at index #0
Internal pubkey: f30544d6009c8d8d94f5d030b2e844b1a3ca036255161c479db1cca5b374dd1c
1 spending argument present
- 1+ spend arguments; TAPSCRIPT mode
2 scripts:
- #0: 0164b175209997a497d964fc1a62885b05a51166a65a90df00492c8d7cf61d6accf54803beac
warning: ambiguous input 1 is interpreted as a numeric value; use OP_1 to force into opcode
warning: ambiguous input 5 is interpreted as a numeric value; use OP_5 to force into opcode
- #1: 204edfcf9dfe6c0b5c83d1ab3f78d1b39a46ebac6798e08e19761f5ed89ec83c10ac006353036f72645151011e1e6170706c69636174696f6e2f6a736f6e3b636861727365743d7574662d385155014b4b7b73656e6465723a2022346564666366396466653663306235633833643161623366373864316233396134366562616336373938653038653139373631663565643839656338336331302268
Script #0 leaf hash = TapLeaf<<0xc0 || 0164b175209997a497d964fc1a62885b05a51166a65a90df00492c8d7cf61d6accf54803beac>>
 → 0ce303b5acc6a0e96a3cdf926242938f6c238b16c04c2a428b44b2634ca6613c
Script #1 leaf hash = TapLeaf<<0xc0 || 204edfcf9dfe6c0b5c83d1ab3f78d1b39a46ebac6798e08e19761f5ed89ec83c10ac006353036f72645151011e1e6170706c69636174696f6e2f6a736f6e3b636861727365743d7574662d385155014b4b7b73656e6465723a2022346564666366396466653663306235633833643161623366373864316233396134366562616336373938653038653139373631663565643839656338336331302268>>
 → 318d714b33394c4281b188ea3d3350d92011c8599550bae06813dbcec033c33b
Branch (#0, #1)
 → df0a2fb8098e18bd5083ae8acb41e20aae02daf838245013fa8d2f42cd5cf78d
Control object = (leaf), (internal pubkey = f30544d6009c8d8d94f5d030b2e844b1a3ca036255161c479db1cca5b374dd1c), ...
... with proof -> f30544d6009c8d8d94f5d030b2e844b1a3ca036255161c479db1cca5b374dd1c0ce303b5acc6a0e96a3cdf926242938f6c238b16c04c2a428b44b2634ca6613c
Tweak value = TapTweak(f30544d6009c8d8d94f5d030b2e844b1a3ca036255161c479db1cca5b374dd1c || df0a2fb8098e18bd5083ae8acb41e20aae02daf838245013fa8d2f42cd5cf78d) = c9aac2f9b79c96377c9aa3d66bdef93a3bd86fa68cfbf05128bceb2db0fde869
Tweaked pubkey = c9c115285680e6dabffd671e807291497aaa5109edfc7437814679c2aa429bc9 (even)
Pubkey matches the scriptPubKey of the input transaction's output #0
Resulting Bech32m address: bcrt1pe8q322zksrnd40lavu0gqu53f9a255gfah78gdupgeuu92jzn0ysnry0c9
Final control object = c0f30544d6009c8d8d94f5d030b2e844b1a3ca036255161c479db1cca5b374dd1c0ce303b5acc6a0e96a3cdf926242938f6c238b16c04c2a428b44b2634ca6613c
Adding selected script to taproot inputs: 204edfcf9dfe6c0b5c83d1ab3f78d1b39a46ebac6798e08e19761f5ed89ec83c10ac006353036f72645151011e1e6170706c69636174696f6e2f6a736f6e3b636861727365743d7574662d385155014b4b7b73656e6465723a2022346564666366396466653663306235633833643161623366373864316233396134366562616336373938653038653139373631663565643839656338336331302268
 → 4c9d204edfcf9dfe6c0b5c83d1ab3f78d1b39a46ebac6798e08e19761f5ed89ec83c10ac006353036f72645151011e1e6170706c69636174696f6e2f6a736f6e3b636861727365743d7574662d385155014b4b7b73656e6465723a2022346564666366396466653663306235633833643161623366373864316233396134366562616336373938653038653139373631663565643839656338336331302268
appending control object to taproot input stack: c0f30544d6009c8d8d94f5d030b2e844b1a3ca036255161c479db1cca5b374dd1c0ce303b5acc6a0e96a3cdf926242938f6c238b16c04c2a428b44b2634ca6613c
Tapscript spending witness: [
 "204edfcf9dfe6c0b5c83d1ab3f78d1b39a46ebac6798e08e19761f5ed89ec83c10ac006353036f72645151011e1e6170706c69636174696f6e2f6a736f6e3b636861727365743d7574662d385155014b4b7b73656e6465723a2022346564666366396466653663306235633833643161623366373864316233396134366562616336373938653038653139373631663565643839656338336331302268",
 "c0f30544d6009c8d8d94f5d030b2e844b1a3ca036255161c479db1cca5b374dd1c0ce303b5acc6a0e96a3cdf926242938f6c238b16c04c2a428b44b2634ca6613c",
]
input tx index = 0; tx input vout = 0; value = 100000
got witness stack of size 2
34 bytes (v0=P2WSH, v1=taproot/tapscript)
Taproot commitment:
- control  = c0f30544d6009c8d8d94f5d030b2e844b1a3ca036255161c479db1cca5b374dd1c0ce303b5acc6a0e96a3cdf926242938f6c238b16c04c2a428b44b2634ca6613c
- program  = c9c115285680e6dabffd671e807291497aaa5109edfc7437814679c2aa429bc9
- script   = 204edfcf9dfe6c0b5c83d1ab3f78d1b39a46ebac6798e08e19761f5ed89ec83c10ac006353036f72645151011e1e6170706c69636174696f6e2f6a736f6e3b636861727365743d7574662d385155014b4b7b73656e6465723a2022346564666366396466653663306235633833643161623366373864316233396134366562616336373938653038653139373631663565643839656338336331302268
- path len = 1
- p        = f30544d6009c8d8d94f5d030b2e844b1a3ca036255161c479db1cca5b374dd1c
- q        = c9c115285680e6dabffd671e807291497aaa5109edfc7437814679c2aa429bc9
- k        = 3bc333c0cedb1368e0ba509559c81120d950333dea88b181424c39334b718d31          (tap leaf hash)
  (TapLeaf(0xc0 || 204edfcf9dfe6c0b5c83d1ab3f78d1b39a46ebac6798e08e19761f5ed89ec83c10ac006353036f72645151011e1e6170706c69636174696f6e2f6a736f6e3b636861727365743d7574662d385155014b4b7b73656e6465723a2022346564666366396466653663306235633833643161623366373864316233396134366562616336373938653038653139373631663565643839656338336331302268))
valid script
- generating prevout hash from 1 ins
[+] COutPoint(e18be5221d, 0)
SignatureHashSchnorr(in_pos=0, hash_type=00)
- tapscript sighash
sighash (little endian) = 6ef1228025608366fa171329aca432a1fdde2def151d7afbd115e7eaa5121a33
sighash: 6ef1228025608366fa171329aca432a1fdde2def151d7afbd115e7eaa5121a33
privkey: 81b637d8fcd2c6da6359e6963113a1170de795e4b725b84d1e0b4cfd9ec58ce9
pubkey: 4edfcf9dfe6c0b5c83d1ab3f78d1b39a46ebac6798e08e19761f5ed89ec83c10
signature: 27ddfed4b5796ed22b6e8c895cc7ff4ea983ab4c8c734d63d899e7f944adbee06a1737f1c27ababf7fcedef07754614a5f82b3dd316d8ddccba92e033146c71c
Resulting transaction: 02000000000101b26b28f52526ec6a10788a1c7404cc8ea96233c6d71839dce6ae151d22e58be10000000000fdffffff01905f010000000000160014454c36912f68b0ad122511056ecbf973a827dd02034027ddfed4b5796ed22b6e8c895cc7ff4ea983ab4c8c734d63d899e7f944adbee06a1737f1c27ababf7fcedef07754614a5f82b3dd316d8ddccba92e033146c71c9d204edfcf9dfe6c0b5c83d1ab3f78d1b39a46ebac6798e08e19761f5ed89ec83c10ac006353036f72645151011e1e6170706c69636174696f6e2f6a736f6e3b636861727365743d7574662d385155014b4b7b73656e6465723a202234656466636639646665366330623563383364316162336637386431623339613436656261633637393865303865313937363166356564383965633833633130226841c0f30544d6009c8d8d94f5d030b2e844b1a3ca036255161c479db1cca5b374dd1c0ce303b5acc6a0e96a3cdf926242938f6c238b16c04c2a428b44b2634ca6613c00000000
```

tips: your can create Taproot spend locktime script with
```
tap -k81b637d8fcd2c6da6359e6963113a1170de795e4b725b84d1e0b4cfd9ec58ce9 --tx=$tx --txin=$txin $pubkey 2 "${script_alice}" "${script_bob}"  0
```

# 7 broadcast tx and spend script

CLI:
```bash
bcli -rpcuser=rpcuser -rpcpassword=rpcpassword sendrawtransaction 02000000000101b26b28f52526ec6a10788a1c7404cc8ea96233c6d71839dce6ae151d22e58be10000000000fdffffff01905f010000000000160014454c36912f68b0ad122511056ecbf973a827dd02034027ddfed4b5796ed22b6e8c895cc7ff4ea983ab4c8c734d63d899e7f944adbee06a1737f1c27ababf7fcedef07754614a5f82b3dd316d8ddccba92e033146c71c9d204edfcf9dfe6c0b5c83d1ab3f78d1b39a46ebac6798e08e19761f5ed89ec83c10ac006353036f72645151011e1e6170706c69636174696f6e2f6a736f6e3b636861727365743d7574662d385155014b4b7b73656e6465723a202234656466636639646665366330623563383364316162336637386431623339613436656261633637393865303865313937363166356564383965633833633130226841c0f30544d6009c8d8d94f5d030b2e844b1a3ca036255161c479db1cca5b374dd1c0ce303b5acc6a0e96a3cdf926242938f6c238b16c04c2a428b44b2634ca6613c00000000
```

UI terminal:
```bash
sendrawtransaction 02000000000101b26b28f52526ec6a10788a1c7404cc8ea96233c6d71839dce6ae151d22e58be10000000000fdffffff01905f010000000000160014454c36912f68b0ad122511056ecbf973a827dd02034027ddfed4b5796ed22b6e8c895cc7ff4ea983ab4c8c734d63d899e7f944adbee06a1737f1c27ababf7fcedef07754614a5f82b3dd316d8ddccba92e033146c71c9d204edfcf9dfe6c0b5c83d1ab3f78d1b39a46ebac6798e08e19761f5ed89ec83c10ac006353036f72645151011e1e6170706c69636174696f6e2f6a736f6e3b636861727365743d7574662d385155014b4b7b73656e6465723a202234656466636639646665366330623563383364316162336637386431623339613436656261633637393865303865313937363166356564383965633833633130226841c0f30544d6009c8d8d94f5d030b2e844b1a3ca036255161c479db1cca5b374dd1c0ce303b5acc6a0e96a3cdf926242938f6c238b16c04c2a428b44b2634ca6613c00000000
```

Output:

```bash
6f37e8e72161c58c28ca4bb909a11bfd745d78227a6b40de2bde9d9cd6604e2f
```

Let's save the output:

```bash
export spend_tx="6f37e8e72161c58c28ca4bb909a11bfd745d78227a6b40de2bde9d9cd6604e2f"
```

# 8 use message product schnorr signature for ggx pubkey

```bash
git clone https://github.com/BitPolito/schnorr-sig.git
cd schnorr-sig
```

edit users.json and save:

```json
{
    "$schema": "./users_schema.json",
    "users": [
        {
            "privateKey": "81b637d8fcd2c6da6359e6963113a1170de795e4b725b84d1e0b4cfd9ec58ce9",
            "publicKey": "4edfcf9dfe6c0b5c83d1ab3f78d1b39a46ebac6798e08e19761f5ed89ec83c10"
        }
    ]
}
```

`python3 schnorr_sign.py  -m hello`

```
> Message = 2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824
> Signature = 160bee884c11921fd3176eff2286f891f36a283b9c6639bbf3ffdba91af7576188cfc14339e3a0e978325023cc4a984fede6d53c6d1bf3aef8fabca33cc8266b
```

Let's save it

```bash
export message="2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
export signature="160bee884c11921fd3176eff2286f891f36a283b9c6639bbf3ffdba91af7576188cfc14339e3a0e978325023cc4a984fede6d53c6d1bf3aef8fabca33cc8266b"
```

# 9 run verify program

```bash
cargo run --release -- --utxo-txid $tx_hash --utxo-txid-index 0  --pubkey-locktime "9997a497d964fc1a62885b05a51166a65a90df00492c8d7cf61d6accf54803be"  --pubkey-ggx "4edfcf9dfe6c0b5c83d1ab3f78d1b39a46ebac6798e08e19761f5ed89ec83c10"  --locktime 100  --message $message  --signature $signature  --spend-utxo-txid  $spend_tx
```

```bash
######### step 1 Verify the UTXO was included in the Bitcoin block number
@@@ utxo no confirm greater than 10, confirmations is 0
@@@ utxo output is in blockchain

######### step 2 Verify Script #0 and Script #1 are included in the UTXO
@@@ product addr from two script hash bcrt1pe8q322zksrnd40lavu0gqu53f9a255gfah78gdupgeuu92jzn0ysnry0c9
@@@ address in utxo script_pubkey bcrt1pe8q322zksrnd40lavu0gqu53f9a255gfah78gdupgeuu92jzn0ysnry0c9
@@@ script in utxo

######### step 3. Verify Spend Script #0 is a Time Lock script
@@@ script1 is not same, this spend utxo is not use locktime script 

######### step 4  Verify Spend Script #1 checks the correct Schnorr signature and has the correct pub key
@@@ ggx script verify input_schnorr_signature and pubkey is match for giving message and signature
@@@ script for ggx is same with spend utxo
@@@ ggx script verify signature and pubkey is match with spend utxo

```
