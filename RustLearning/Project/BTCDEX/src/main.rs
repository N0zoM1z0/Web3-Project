use bitcoin::blockdata::opcodes;
use bitcoin::blockdata::script::{self, Builder, Script};
use bitcoin::blockdata::transaction::{OutPoint, Sequence, Transaction, TxIn, TxOut};
use bitcoin::hashes::{hash160, Hash};
use bitcoin::secp256k1::{rand::{self, RngCore}, Secp256k1, SecretKey, PublicKey};
use bitcoin::util::address::Address;
use bitcoin::util::amount::Amount;
use bitcoin::network::constants::Network;
use bitcoin::consensus::encode; // 用于序列化
use bitcoin::locktime::PackedLockTime; // 在 0.29 中，时间锁用 PackedLockTime

use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- 配置 ---
    let network = Network::Testnet; // 使用测试网
    let secp = Secp256k1::new();

    // --- 模拟参与者 ---
    // Alice (发起方)
    let alice_privkey = SecretKey::new(&mut rand::thread_rng());
    let alice_pubkey = PublicKey::from_secret_key(&secp, &alice_privkey);
    println!("Alice PubKey: {}", alice_pubkey);

    // Bob (接收方)
    let bob_privkey = SecretKey::new(&mut rand::thread_rng());
    let bob_pubkey = PublicKey::from_secret_key(&secp, &bob_privkey);
    println!("Bob PubKey: {}", bob_pubkey);

    // --- 原子交换参数 ---
    // 1. 秘密和哈希 (Alice 生成)
    let mut preimage = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut preimage);
    let preimage_hash = hash160::Hash::hash(&preimage);
    println!("Preimage: {}", hex::encode(preimage));
    println!("Preimage Hash (H160): {}", preimage_hash);

    // 2. 时间锁 (例如，24小时后 Alice 可以取回)
    // 注意：比特币的时间锁可以是区块高度或 Unix 时间戳
    // 这里我们用一个相对时间锁（Sequence number）或者绝对时间锁 (nLockTime)
    // 为了简单，我们用绝对时间锁（nLockTime），需要一个未来的区块高度或时间戳
    // 假设我们用未来的区块高度，比如当前高度 + 144 个块 (约一天)
    // let refund_locktime_value: u32 = 2_500_000; // 假设是未来的某个区块高度
    // 或者使用时间戳 (必须大于 500,000,000)
    let current_timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    // 时间戳需要比当前块的时间中值 (MTP) 大
    let refund_timestamp = (current_timestamp + 24 * 60 * 60) as u32; // 24 小时后的 Unix 时间戳
    let refund_locktime = PackedLockTime(refund_timestamp);
    println!("Refund LockTime (Timestamp): {}", refund_timestamp);


    // --- 构建 HTLC 脚本 ---
    // P2WSH (推荐) 或 P2SH
    // Script:
    // OP_IF
    //   # Bob 的花费路径 (提供 preimage)
    //   OP_HASH160 <hash160(preimage)> OP_EQUALVERIFY
    //   <Bob's PubKey> OP_CHECKSIG
    // OP_ELSE
    //   # Alice 的退款路径 (等待时间锁)
    //   <Refund LockTime> OP_CHECKLOCKTIMEVERIFY OP_DROP
    //   <Alice's PubKey> OP_CHECKSIG
    // OP_ENDIF
    let htlc_script = Builder::new()
        .push_opcode(opcodes::all::OP_IF)
            .push_opcode(opcodes::all::OP_HASH160)
            .push_slice(&preimage_hash.into_inner()) // 推入 preiamge 的 HASH160
            .push_opcode(opcodes::all::OP_EQUALVERIFY)
            .push_key(&bob_pubkey) // 推入 Bob 的公钥
            .push_opcode(opcodes::all::OP_CHECKSIG)
        .push_opcode(opcodes::all::OP_ELSE)
            // .push_int(refund_locktime_value as i64) // 推入时间锁值 (如果是区块高度)
            .push_int(refund_locktime.to_consensus_u32() as i64) // 推入时间锁值 (时间戳或高度)
            .push_opcode(opcodes::all::OP_CHECKLOCKTIMEVERIFY) // 检查交易的 nLockTime
            .push_opcode(opcodes::all::OP_DROP) // 丢弃时间锁值
            .push_key(&alice_pubkey) // 推入 Alice 的公钥
            .push_opcode(opcodes::all::OP_CHECKSIG)
        .push_opcode(opcodes::all::OP_ENDIF)
        .into_script();

    println!("HTLC Redeem Script:\n{}", htlc_script);

    // --- 创建 P2WSH 地址 ---
    // P2WSH 地址是版本0 + witness program (脚本的 SHA256)
    let p2wsh_address = Address::p2wsh(&htlc_script, network);
    println!("P2WSH Address: {}", p2wsh_address);


    // --- 构建资金交易 (Funding Transaction) ---
    // Alice 需要花费她自己的一个 UTXO 来创建这个 HTLC 输出

    // 假设 Alice 有一个 UTXO (你需要从你的钱包或测试网水龙头获取真实的 UTXO)
    let funding_utxo_txid_str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"; // 示例 TXID
    let funding_utxo_vout: u32 = 0; // 示例 Vout
    let funding_utxo_amount = Amount::from_btc(0.01)?; // 示例 UTXO 金额
    let amount_to_lock = Amount::from_btc(0.005)?; // Alice 想要锁定的金额
    let fee = Amount::from_sat(500); // 交易费，需要估算

    let funding_txid = bitcoin::Txid::from_str(funding_utxo_txid_str)?;
    let funding_outpoint = OutPoint { txid: funding_txid, vout: funding_utxo_vout };

    // 输入: 花费 Alice 的 UTXO
    let input = TxIn {
        previous_output: funding_outpoint,
        script_sig: Script::new(), // 对于 P2WPKH/P2SH-P2WPKH/P2WSH 输入，script_sig 通常为空或包含 P2SH redeemScript
        sequence: Sequence::MAX, // 通常设为 RBF 或默认值
        witness: vec![], // 签名将在后面添加 (这里只是构建，不签名)
    };

    // 输出 0: HTLC 输出，发送到 P2WSH 地址
    let htlc_output = TxOut {
        value: amount_to_lock.as_sat(),
        script_pubkey: p2wsh_address.script_pubkey(), // P2WSH scriptPubKey
    };

    // 输出 1: 找零 (如果需要)
    let change_amount = funding_utxo_amount - amount_to_lock - fee;
    if change_amount < Amount::ZERO {
        return Err("Insufficient funds for transaction".into());
    }

    // 假设 Alice 的找零地址也是 P2WPKH
    let alice_change_address = Address::p2wpkh(&alice_pubkey, network)?;
    let change_output = TxOut {
        value: change_amount.as_sat(),
        script_pubkey: alice_change_address.script_pubkey(),
    };

    // 创建交易
    let mut funding_transaction = Transaction {
        version: 2, // 常用版本
        lock_time: PackedLockTime::ZERO, // 这个交易本身没有时间锁
        input: vec![input],
        output: vec![htlc_output],
    };

    if change_amount > Amount::from_sat(546) { // 检查粉尘阈值
        funding_transaction.output.push(change_output);
    }

    println!("\n--- Funding Transaction (Unsigned) ---");
    println!("{:#?}", funding_transaction);

    // --- 序列化交易 (用于广播或签名) ---
    let serialized_tx = encode::serialize(&funding_transaction);
    println!("\nSerialized Funding Transaction (Hex):");
    println!("{}", hex::encode(&serialized_tx));


    // --- 下一步 (非常重要！) ---
    // 1. **签名**: Alice 需要使用她的私钥为 funding_transaction 的输入签名。
    //    签名过程对于 P2WPKH/P2SH/P2WSH 输入是不同的，需要计算 Sighash。
    //    这部分比较复杂，通常会使用更高层的库（如 BDK）或仔细处理 sighash 计算和 witness 构建。
    //    使用 bitcoin 0.29 和 secp256k1 0.27 直接签名 P2WPKH 输入大概是：
    //    - 计算 sighash (e.g., using `bitcoin::util::sighash::sighash_for_witness_input`)
    //    - 用 `secp.sign_ecdsa(...)` 签名 sighash
    //    - 将签名和公钥放入 `TxIn.witness`
    //
    // 2. **广播**: 将签好名的交易广播到比特币网络。
    //
    // 3. **通知 Bob**: Alice 将 funding transaction 的 TXID 和 HTLC 脚本告知 Bob。
    //
    // 4. **Bob 的操作**: Bob 验证交易和脚本后，会在他那端（可能是另一条链，或也是比特币）创建类似的 HTLC。
    //
    // 5. **认领/退款**:
    //    - **Bob 认领**: Bob 广播一个交易，花费 Alice 创建的 HTLC 输出。这个交易的 witness 需要包含 `preimage`、Bob 的签名和一个 `OP_TRUE`（表示选择 `OP_IF` 分支）。交易的 `nLockTime` 必须小于 HTLC 脚本中的 `OP_CHECKLOCKTIMEVERIFY` 值。
    //    - **Alice 退款**: 如果 Bob 在时间锁到期前没有认领，Alice 可以广播一个交易来花费 HTLC 输出。这个交易的 `nLockTime` 必须 **大于等于** HTLC 脚本中的 `OP_CHECKLOCKTIMEVERIFY` 值。交易的 witness 需要包含 Alice 的签名和一个 `OP_FALSE`（表示选择 `OP_ELSE` 分支）。

    println!("\n--- Notes ---");
    println!("- This script only creates the *funding* transaction structure for Alice.");
    println!("- Signing the input is a complex step not shown here. Requires Alice's private key and correct sighash calculation.");
    println!("- Use a real UTXO from Testnet faucet for actual testing.");
    println!("- A full swap requires communication, monitoring, and handling Bob's side.");
    println!("- Consider using higher-level libraries like BDK for easier wallet management and signing if needed.");

    Ok(())
}