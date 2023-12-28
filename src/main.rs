use bitcoin::blockdata::script::Script;
use bitcoin::blockdata::script::ScriptBuf;
use bitcoin::hashes::Hash;

use bitcoin::key::Secp256k1;
use bitcoin::secp256k1::schnorr::Signature;

use bitcoin::sighash::Prevouts;
use bitcoin::sighash::ScriptPath;

use bitcoin::taproot::TapTweakHash;
use bitcoin::taproot::TaprootBuilder;
use bitcoin::TapSighashType;
use bitcoin::Transaction;
use bitcoin::Txid;
use bitcoin::{Address, ScriptHash};
use bitcoin::{Network, XOnlyPublicKey};
use bitcoincore_rpc::bitcoin;
use bitcoincore_rpc::bitcoin::key::UntweakedPublicKey;
use bitcoincore_rpc::bitcoin::taproot::LeafVersion;
use bitcoincore_rpc::bitcoin::TapNodeHash;
use bitcoincore_rpc::{Auth, Client, RpcApi};

use secp256k1::Message;
use std::ops::Index;
use std::process::Command;
use std::str::FromStr;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// utxo txid
    #[arg(short, long)]
    utxo_txid: String,

    /// utxo tx index
    #[arg(long)]
    utxo_txid_index: usize,

    /// pubkey for script locktime
    #[arg(long)]
    pubkey_locktime: String,

    /// pubkey for script unlock by ggx
    #[arg(long)]
    pubkey_ggx: String,

    /// locktime
    #[arg(short, long)]
    locktime: u32,

    /// message for check schnorr signature
    #[arg(short, long)]
    message: String,

    /// schnorr signature ggx pubkey
    #[arg(long)]
    signature: String,

    /// txid which spend utxo
    #[arg(long)]
    spend_utxo_txid: String,
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

fn get_path1_locktime_script(lock_time: u32, pubkey: &str) -> ScriptBuf {
    let script = format!("{lock_time} OP_CHECKLOCKTIMEVERIFY OP_DROP {pubkey} OP_CHECKSIG");
    let parts = script.split(" ");

    let output = Command::new("btcc")
        .args(parts.collect::<Vec<_>>())
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());

    let mut hex_script1 = String::from_utf8_lossy(&output.stdout).into_owned();

    trim_newline(&mut hex_script1);

    let script1 = hex::decode(&hex_script1).expect("Decoding failed");

    let s1 = Script::from_bytes(&script1);

    s1.into()
}

fn get_path1_locktime_script_hash(lock_time: u32, pubkey: &str) -> ScriptHash {
    let script = format!("{lock_time} OP_CHECKLOCKTIMEVERIFY OP_DROP {pubkey} OP_CHECKSIG");
    let parts = script.split(" ");

    let output = Command::new("btcc")
        .args(parts.collect::<Vec<_>>())
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());

    let mut hex_script1 = String::from_utf8_lossy(&output.stdout).into_owned();

    trim_newline(&mut hex_script1);

    let script1 = hex::decode(&hex_script1).expect("Decoding failed");

    let s1 = Script::from_bytes(&script1);

    s1.script_hash()
}

fn get_path2_ggx_script(pubkey: &str) -> ScriptBuf {
    let script = format!("{pubkey} OP_CHECKSIG OP_FALSE OP_IF OP_3 6f7264 OP_1 1 0x1e 6170706c69636174696f6e2f6a736f6e3b636861727365743d7574662d38 OP_1 5 0x4b   7b73656e6465723a20223465646663663964666536633062356338336431616233663738643162333961343665626163363739386530386531393736316635656438396563383363313022 OP_ENDIF");
    let parts = script.split(" ");

    let output = Command::new("btcc")
        .args(parts.collect::<Vec<_>>())
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());

    let mut hex_script1 = String::from_utf8_lossy(&output.stdout).into_owned();

    trim_newline(&mut hex_script1);

    let script1 = hex::decode(&hex_script1).expect("Decoding failed");

    let s1 = Script::from_bytes(&script1);

    s1.into()
}

fn get_path2_ggx_script_hash(pubkey: &str) -> ScriptHash {
    let script = format!("{pubkey} OP_CHECKSIG OP_FALSE OP_IF OP_3 6f7264 OP_1 1 0x1e 6170706c69636174696f6e2f6a736f6e3b636861727365743d7574662d38 OP_1 5 0x4b   7b73656e6465723a20223465646663663964666536633062356338336431616233663738643162333961343665626163363739386530386531393736316635656438396563383363313022 OP_ENDIF");
    let parts = script.split(" ");

    let output = Command::new("btcc")
        .args(parts.collect::<Vec<_>>())
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());

    let mut hex_script1 = String::from_utf8_lossy(&output.stdout).into_owned();

    trim_newline(&mut hex_script1);

    let script1 = hex::decode(&hex_script1).expect("Decoding failed");

    let s1 = Script::from_bytes(&script1);

    s1.script_hash()
}

fn get_rawtx(rpc: &Client, txid: &str) -> Transaction {
    let mut decoded = [0u8; 32];
    hex::decode_to_slice(txid, &mut decoded).expect("Decoding failed");
    decoded.reverse();

    let txid = Txid::from_byte_array(decoded);
    let raw_tx = rpc.get_raw_transaction(&txid, None).unwrap();

    raw_tx
}

fn check_utxo_in_block(rpc: &Client, hex_tx_id: &str, index: usize) {
    let mut decoded = [0u8; 32];
    hex::decode_to_slice(hex_tx_id, &mut decoded).expect("Decoding failed");
    decoded.reverse();

    let txid = Txid::from_byte_array(decoded);

    let tx = rpc.get_transaction(&txid, None).unwrap();

    if tx.info.confirmations > 10 {
        println!(
            "@@@ utxo in block and confirmations is {:?}",
            tx.info.confirmations
        );
    } else {
        println!(
            "@@@ utxo no confirm greater than 10, confirmations is {:?}",
            tx.info.confirmations
        );
    }

    let raw_tx = rpc.get_raw_transaction(&txid, None).unwrap();

    if raw_tx.output.len() <= index {
        println!("@@@ utxo index is not in blockchain");
    } else {
        println!("@@@ utxo output is in blockchain");
    }
}

fn check_script_in_utxo(
    rpc: &Client,
    hex_tx_id: &str,
    _utxo_index: usize,
    locktime: u32,
    pubkey_locktime: &str,
    pubkey_ggx: &str,
) {
    let s1 = get_path1_locktime_script(locktime, pubkey_locktime);
    let s2 = get_path2_ggx_script(pubkey_ggx);

    let root = TaprootBuilder::new();
    let root = root.add_leaf(1, s1.clone()).unwrap();
    let root = root.add_leaf(1, s2.clone()).unwrap();

    let _tree = root.try_into_taptree().unwrap();

    let hash1 = TapNodeHash::from_script(&s1, LeafVersion::TapScript);
    let hash2 = TapNodeHash::from_script(&s2, LeafVersion::TapScript);
    let root_hash = TapNodeHash::from_node_hashes(hash1, hash2);

    let internal_pubkey = "f30544d6009c8d8d94f5d030b2e844b1a3ca036255161c479db1cca5b374dd1c";
    let un_tweak_key = UntweakedPublicKey::from_str(internal_pubkey).unwrap();
    let _un_tweak_hash = TapTweakHash::from_key_and_tweak(un_tweak_key, Some(root_hash));

    let secp = Secp256k1::new();

    let addr = Address::p2tr(&secp, un_tweak_key, Some(root_hash), Network::Regtest);
    println!("@@@ product addr from two script hash {:?}", addr); //check addr equ utxo scriptPubKey.address

    let mut decoded = [0u8; 32];
    hex::decode_to_slice(hex_tx_id, &mut decoded).expect("Decoding failed");
    decoded.reverse();

    let _txid = Txid::from_byte_array(decoded);

    let utxo_raw_tx = get_rawtx(&rpc, hex_tx_id);
    let new_address = Address::from_script(
        utxo_raw_tx.output[0].script_pubkey.as_script(),
        Network::Regtest,
    )
    .unwrap();

    println!("@@@ address in utxo script_pubkey {:?}", new_address);

    if addr == new_address {
        println!("@@@ script in utxo");
    } else {
        println!("@@@ script not in utxo");
    }
}

fn check_spend_uxto_is_locktime(
    rpc: &Client,
    lock_time: u32,
    lock_time_pubkey: &str,
    spend_tx_id: &str,
) {
    //let utxo_raw_tx = get_rawtx(rpc, utxo_tx_id);
    let spend_raw_tx = get_rawtx(rpc, spend_tx_id);

    //script_pubkey
    let t = &spend_raw_tx.input[0].witness.index(1);

    //

    let witness_script = Script::from_bytes(t);

    let script1 = get_path1_locktime_script_hash(lock_time, lock_time_pubkey);

    if witness_script.script_hash() == script1 {
        println!("@@@ script1 is same, this spend utxo is use locktime script");
    } else {
        println!("@@@ script1 is not same, this spend utxo is not use locktime script ");
    }
}

fn check_spend_uxto_is_by_ggx_pubkey(
    rpc: &Client,
    utxo_tx_id: &str,
    input_index: usize,
    pub_key: &str,
    input_message: &str,
    input_schnorr_signature: &str,
    spend_tx_id: &str,
) {
    let secp = Secp256k1::new();
    let mut decoded = [0u8; 32];
    hex::decode_to_slice(pub_key, &mut decoded).expect("Decoding failed");

    let pubkey = XOnlyPublicKey::from_slice(&decoded).unwrap();

    {
        let input_message = hex::decode(input_message).expect("Decoding input_message failed");
        let input_schnorr_signature =
            hex::decode(input_schnorr_signature).expect("Decoding input_schnorr_signature failed");

        let input_message = Message::from_digest_slice(input_message.as_slice()).unwrap();
        let input_schnorr_signature =
            Signature::from_slice(input_schnorr_signature.as_slice()).unwrap();
        let rt = secp.verify_schnorr(&input_schnorr_signature, &input_message, &pubkey);
        if rt.is_ok() {
            println!("@@@ ggx script verify input_schnorr_signature and pubkey is match for giving message and signature");
        } else {
            println!("@@@ ggx script verify input_schnorr_signature and pubkey is not match for giving message and signature");
        }
    }

    let utxo_raw_tx = get_rawtx(rpc, utxo_tx_id);
    let spend_raw_tx = get_rawtx(rpc, spend_tx_id);

    //script_pubkey
    let t = &spend_raw_tx.input[0].witness.index(1);

    //

    let witness_script = Script::from_bytes(t);

    let script_hash = get_path2_ggx_script_hash(pub_key);

    if witness_script.script_hash() == script_hash {
        println!("@@@ script for ggx is same with spend utxo");
    } else {
        println!("@@@ script for ggx is not the same with spend utxo");
        return;
    }

    let prevouts = vec![&utxo_raw_tx.output[input_index]];
    let mut sighash_cache = bitcoin::sighash::SighashCache::new(&spend_raw_tx);

    // taproot_key_spend_signature_hash is the crucial part
    let signature_hash = sighash_cache
        .taproot_script_spend_signature_hash(
            0,
            &Prevouts::All(&prevouts),
            ScriptPath::with_defaults(witness_script),
            TapSighashType::Default,
        )
        .unwrap();

    let message_sighash = Message::from_digest_slice(&signature_hash.to_byte_array()).unwrap();

    let sig = &spend_raw_tx.input[0].witness.index(0);
    let signature = Signature::from_slice(sig).unwrap();

    let rt = secp.verify_schnorr(&signature, &message_sighash, &pubkey);
    if rt.is_ok() {
        println!("@@@ ggx script verify signature and pubkey is match with spend utxo");
    } else {
        println!("@@@ ggx script verify signature and pubkey is not match with spend utxo");
    }
}

fn main() {
    let args = Args::parse();

    let rpc = Client::new(
        "http://localhost:8332",
        Auth::UserPass("rpcuser".to_string(), "rpcpassword".to_string()),
    )
    .unwrap();

    println!("\n######### step 1 Verify the UTXO was included in the Bitcoin block number");
    check_utxo_in_block(&rpc, &args.utxo_txid, args.utxo_txid_index);

    println!("\n######### step 2 Verify Script #0 and Script #1 are included in the UTXO");
    check_script_in_utxo(
        &rpc,
        &args.utxo_txid,
        args.utxo_txid_index,
        args.locktime,
        &args.pubkey_locktime,
        &args.pubkey_ggx,
    );

    println!("\n######### step 3. Verify Spend Script #0 is a Time Lock script");
    check_spend_uxto_is_locktime(
        &rpc,
        args.locktime,
        &args.pubkey_locktime,
        &args.spend_utxo_txid,
    );

    println!("\n######### step 4  Verify Spend Script #1 checks the correct Schnorr signature and has the correct pub key");
    check_spend_uxto_is_by_ggx_pubkey(
        &rpc,
        &args.utxo_txid,
        args.utxo_txid_index,
        &args.pubkey_ggx,
        &args.message,
        &args.signature,
        &args.spend_utxo_txid,
    );
}
