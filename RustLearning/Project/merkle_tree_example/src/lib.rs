use sha2::{Digest, Sha256};
use std::fmt;

// 定义哈希类型，方便使用 (SHA256 输出是 32 字节)
pub type Hash = [u8; 32];

// Merkle Tree 结构体
#[derive(Debug, Clone)]
pub struct MerkleTree {
    root: Option<Hash>,
    // 注意：为了简化，我们这里只存储根。
    // 如果需要生成 Merkle Proof，则需要存储更多中间节点或叶子节点信息。
}

// Bitcoin 使用双 SHA256 哈希
fn double_sha256(data: &[u8]) -> Hash {
    let hash1 = Sha256::digest(data);
    let hash2 = Sha256::digest(&hash1);
    hash2.into() // Digest 返回 GenericArray，我们转换为 [u8; 32]
}

impl MerkleTree {
    /// 创建一个新的 Merkle Tree
    ///
    /// # Arguments
    ///
    /// * `data` - 一个包含需要哈希的数据项的切片。每个数据项需要能被转换成字节引用 (`AsRef<[u8]>`)。
    ///            例如：Vec<String>, Vec<Vec<u8>>, Vec<&[u8]> 等。
    ///
    /// # Returns
    ///
    /// 如果输入数据为空，返回 `None`。否则，返回包含计算出的 Merkle Root 的 `MerkleTree` 实例。
    pub fn new<T: AsRef<[u8]>>(data: &[T]) -> Self {
        if data.is_empty() {
            return MerkleTree { root: None };
        }

        // 1. 计算所有叶子节点的哈希 (使用双 SHA256)
        let mut current_level: Vec<Hash> = data
            .iter()
            .map(|item| double_sha256(item.as_ref()))
            .collect();

        // 2. 逐层构建树，直到只剩下一个根节点
        while current_level.len() > 1 {
            // 处理奇数个节点的情况：复制最后一个节点
            if current_level.len() % 2 != 0 {
                // 安全地克隆最后一个元素，因为我们已经检查过不为空
                let last_hash = current_level.last().unwrap().clone();
                current_level.push(last_hash);
            }

            let mut next_level = Vec::new();
            // 每次取两个哈希进行组合和哈希
            for chunk in current_level.chunks_exact(2) {
                let left = &chunk[0];
                let right = &chunk[1];

                // 拼接两个哈希
                let mut combined_data = Vec::with_capacity(64);
                combined_data.extend_from_slice(left);
                combined_data.extend_from_slice(right);

                // 计算父节点的哈希 (双 SHA256)
                let parent_hash = double_sha256(&combined_data);
                next_level.push(parent_hash);
            }
            // 更新当前层为新计算出的上一层
            current_level = next_level;
        }

        // 最终 current_level 只包含一个元素，即 Merkle Root
        // 我们已经检查过 data 不为空，所以这里 current_level[0] 一定存在
        MerkleTree {
            root: Some(current_level[0]),
        }
    }

    /// 获取 Merkle Root 的哈希值 (如果存在)
    pub fn root(&self) -> Option<&Hash> {
        self.root.as_ref()
    }

    /// 以十六进制字符串形式获取 Merkle Root (如果存在)
    pub fn root_hex(&self) -> Option<String> {
        self.root.map(hex::encode)
    }
}

// 为 Hash 类型实现 Display，方便打印
impl fmt::Display for MerkleTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.root_hex() {
            Some(hex_root) => write!(f, "MerkleTree(root: {})", hex_root),
            None => write!(f, "MerkleTree(empty)"),
        }
    }
}


// --- 测试模块 ---
#[cfg(test)]
mod tests {
    use super::*; // 导入父模块的所有内容

    // Helper 函数用于测试
    fn get_expected_hash(hex_str: &str) -> Hash {
        let bytes = hex::decode(hex_str).expect("Invalid hex string");
        bytes.try_into().expect("Hex string doesn't represent 32 bytes")
    }

    #[test]
    fn test_empty_tree() {
        let data: Vec<String> = vec![];
        let tree = MerkleTree::new(&data);
        assert!(tree.root().is_none());
        assert_eq!(format!("{}", tree), "MerkleTree(empty)");
    }

    #[test]
    fn test_single_transaction() {
        // 单个交易的 Merkle Root 就是其自身的双 SHA256 哈希
        let tx1 = "transaction_1";
        let expected_root = double_sha256(tx1.as_bytes());

        let tree = MerkleTree::new(&[tx1]);
        assert_eq!(tree.root(), Some(&expected_root));
        println!("Single Tx Tree: {}", tree); // 打印 Merkle Tree
    }

    #[test]
    fn test_two_transactions() {
        let tx1 = "tx_a";
        let tx2 = "tx_b";

        let h1 = double_sha256(tx1.as_bytes());
        let h2 = double_sha256(tx2.as_bytes());

        let mut combined = Vec::new();
        combined.extend_from_slice(&h1);
        combined.extend_from_slice(&h2);
        let expected_root = double_sha256(&combined);

        let tree = MerkleTree::new(&[tx1, tx2]);
        assert_eq!(tree.root(), Some(&expected_root));
        println!("Two Tx Tree: {}", tree);
    }

    #[test]
    fn test_three_transactions() {
        // tx1, tx2, tx3 -> H(H(h1+h2) + H(h3+h3))
        let tx1 = "data_item_1";
        let tx2 = "data_item_2";
        let tx3 = "data_item_3";

        let h1 = double_sha256(tx1.as_bytes());
        let h2 = double_sha256(tx2.as_bytes());
        let h3 = double_sha256(tx3.as_bytes());

        // Level 1
        let mut combined12 = Vec::new();
        combined12.extend_from_slice(&h1);
        combined12.extend_from_slice(&h2);
        let h12 = double_sha256(&combined12);

        let mut combined33 = Vec::new(); // 奇数，复制 h3
        combined33.extend_from_slice(&h3);
        combined33.extend_from_slice(&h3);
        let h33 = double_sha256(&combined33);

        // Level 2 (Root)
        let mut combined_root = Vec::new();
        combined_root.extend_from_slice(&h12);
        combined_root.extend_from_slice(&h33);
        let expected_root = double_sha256(&combined_root);

        let tree = MerkleTree::new(&[tx1, tx2, tx3]);
        assert_eq!(tree.root(), Some(&expected_root));
        println!("Three Tx Tree: {}", tree);
    }

    #[test]
    fn test_four_transactions() {
        // tx1, tx2, tx3, tx4 -> H(H(h1+h2) + H(h3+h4))
        let txs = vec!["block_tx_1", "block_tx_2", "block_tx_3", "block_tx_4"];

        let h1 = double_sha256(txs[0].as_bytes());
        let h2 = double_sha256(txs[1].as_bytes());
        let h3 = double_sha256(txs[2].as_bytes());
        let h4 = double_sha256(txs[3].as_bytes());

        // Level 1
        let mut combined12 = Vec::new();
        combined12.extend_from_slice(&h1);
        combined12.extend_from_slice(&h2);
        let h12 = double_sha256(&combined12);

        let mut combined34 = Vec::new();
        combined34.extend_from_slice(&h3);
        combined34.extend_from_slice(&h4);
        let h34 = double_sha256(&combined34);

        // Level 2 (Root)
        let mut combined_root = Vec::new();
        combined_root.extend_from_slice(&h12);
        combined_root.extend_from_slice(&h34);
        let expected_root = double_sha256(&combined_root);

        let tree = MerkleTree::new(&txs);
        assert_eq!(tree.root(), Some(&expected_root));
        println!("Four Tx Tree: {}", tree);

        // 可以用一个已知的 Bitcoin Merkle Root 示例来验证 (如果找到合适的)
        // 例如，创世块只有一个 coinbase 交易
        // Coinbase tx hash (little-endian): 4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b
        // Merkle Root (little-endian): 4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b
        // 注意：Bitcoin 内部通常使用 little-endian 表示哈希，而我们这里计算和存储的是 big-endian (SHA256 标准输出)
        // 如果要精确匹配 Bitcoin 的数据，需要处理字节序转换。
        // 但 Merkle Tree 的 *逻辑* 是正确的。

        // 让我们用创世块的 coinbase 交易哈希 (作为 big-endian 字符串) 来测试
        // 注意：实际应用中，叶子节点是交易 *哈希* (TXID)，而不是交易数据本身。
        // TXID 本身就是双 SHA256 哈希的结果。
        // 为了简化示例，我们直接用字符串作为叶子数据。
        // 如果要模拟 Bitcoin，应该传入 TXID 列表。

        // 假设我们有 TXIDs (已经是双 SHA256 哈希, big-endian 形式)
        let txid1_bytes = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let txid2_bytes = hex::decode("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb").unwrap();

        // 在 Merkle Tree 构建中，叶子节点 *再次* 被哈希 (即使它们已经是哈希了)
        // 这是 Bitcoin 的设计，尽管看起来有点奇怪，但它是标准。
        // **更正**: 仔细回顾比特币协议，叶子节点 *是* 交易的 TXID（双 SHA256 哈希值），
        // 但是在构建 Merkle Tree 时，这些 TXID *不会* 再进行一次哈希。它们直接作为叶子节点。
        // 内部节点才是对其子节点哈希进行拼接和双 SHA256 哈希得到的。

        // 让我们修改 `new` 函数以适应这种情况，或者提供一个变体。
        // 为了保持当前示例的简单性，我们假设输入 `data` 是原始数据，
        // 如果输入已经是哈希，调用者需要注意。

        // 重新验证 test_single_transaction，假设输入是 TXID
        // 创世块 TXID (big-endian): 3ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323af39fab8e1e5e4a
        let genesis_txid_hex = "3ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323af39fab8e1e5e4a";
        let genesis_txid_bytes : Hash = hex::decode(genesis_txid_hex).unwrap().try_into().unwrap();

        // 如果只有一个 TXID，Merkle Root 就是这个 TXID 本身
        // **注意**: 这需要修改 MerkleTree::new 的逻辑，使其不哈希叶子节点。
        // 或者，我们保持当前实现（哈希所有输入），并说明这是针对 *原始数据* 的树。

        // 确认当前实现的行为：
        let tree_from_txid = MerkleTree::new(&[genesis_txid_bytes]); // 这会对 txid 再次哈希
        let expected_root_if_hashed_again = double_sha256(&genesis_txid_bytes);
        assert_eq!(tree_from_txid.root(), Some(&expected_root_if_hashed_again));
        // 这不是比特币创世块的 Merkle Root。

        // --- 如何精确模拟 Bitcoin ---
        // 要精确模拟，你需要:
        // 1. 获取交易的序列化字节数据。
        // 2. 计算每个交易的双 SHA256 哈希，得到 TXID (通常是 little-endian，需要转为 big-endian 用于标准 SHA256 计算，或使用 little-endian 的哈希库)。
        // 3. 将这些 TXID (作为 `[u8; 32]`) 传递给一个 *修改版* 的 `MerkleTree::new`，该版本 *不* 对叶子节点进行初始哈希，而是直接使用它们。
        // 4. 内部节点的计算（拼接 + 双 SHA256）保持不变。

        // 让我们快速创建一个 `MerkleTree::from_hashes` 版本来演示
    }

    // 一个接受预计算哈希的版本 (更接近 Bitcoin 的做法)
    impl MerkleTree {
        pub fn from_hashes(hashes: &[Hash]) -> Self {
            if hashes.is_empty() {
                return MerkleTree { root: None };
            }

            let mut current_level: Vec<Hash> = hashes.to_vec(); // 直接使用传入的哈希

            while current_level.len() > 1 {
                if current_level.len() % 2 != 0 {
                    let last_hash = current_level.last().unwrap().clone();
                    current_level.push(last_hash);
                }

                let mut next_level = Vec::new();
                for chunk in current_level.chunks_exact(2) {
                    let left = &chunk[0];
                    let right = &chunk[1];
                    let mut combined_data = Vec::with_capacity(64);
                    combined_data.extend_from_slice(left);
                    combined_data.extend_from_slice(right);
                    let parent_hash = double_sha256(&combined_data);
                    next_level.push(parent_hash);
                }
                current_level = next_level;
            }

            MerkleTree {
                root: Some(current_level[0]),
            }
        }
    }


    #[test]
    fn test_from_hashes_single() {
         // 创世块 TXID (big-endian hex)
        let genesis_txid_hex = "3ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323af39fab8e1e5e4a";
        let genesis_txid_bytes : Hash = hex::decode(genesis_txid_hex).unwrap().try_into().unwrap();

        // 创世块 Merkle Root (与 TXID 相同，因为只有一个交易)
        // 注意 Bitcoin RPC 返回的是 little-endian hex: 4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b
        // 对应的 big-endian hex: 3ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323af39fab8e1e5e4a
        let expected_root_hex = genesis_txid_hex;
        let expected_root_bytes: Hash = hex::decode(expected_root_hex).unwrap().try_into().unwrap();

        let tree = MerkleTree::from_hashes(&[genesis_txid_bytes]);
        assert_eq!(tree.root(), Some(&expected_root_bytes));
        assert_eq!(tree.root_hex(), Some(expected_root_hex.to_string()));
        println!("Genesis Block Tree (from hash): {}", tree);
    }

     #[test]
    fn test_from_hashes_multiple() {
        // 假设我们有两个 TXID (已经是 Hash 类型)
        let txid1: Hash = hex::decode("1111111111111111111111111111111111111111111111111111111111111111").unwrap().try_into().unwrap();
        let txid2: Hash = hex::decode("2222222222222222222222222222222222222222222222222222222222222222").unwrap().try_into().unwrap();

        // 计算预期根
        let mut combined = Vec::with_capacity(64);
        combined.extend_from_slice(&txid1);
        combined.extend_from_slice(&txid2);
        let expected_root = double_sha256(&combined);

        let tree = MerkleTree::from_hashes(&[txid1, txid2]);
        assert_eq!(tree.root(), Some(&expected_root));
        println!("Two TxID Tree (from hash): {}", tree);
    }

}