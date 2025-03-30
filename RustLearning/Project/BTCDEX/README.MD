# Build a BTC based DEX in Rust
好的！现在我们回到用 Rust 实现一个简单的、基于 Bitcoin 的 DEX（去中心化交易所）这个项目。 让我们逐步推进。

回顾 (Recap):

我们之前讨论过，由于 Bitcoin 本身不支持复杂的智能合约，我们需要使用一些特殊的技术来实现 DEX 的核心功能。 主要思路是利用原子互换（Atomic Swap）来实现不同资产之间的安全交换。 这种DEX的交易对可能是 BTC/某种其他链的Token（例如：BTC/Liquid上的L-BTC)。

关键技术 (Key Technologies):

原子互换 (Atomic Swaps): 允许两个人在没有信任第三方的情况下，安全地交换不同区块链上的资产。

HTLC (Hashed TimeLock Contracts): 原子互换通常使用 HTLC 实现。 HTLC 是一种特殊的智能合约，它使用哈希锁和时间锁来确保交易的原子性。

链下撮合 (Off-Chain Matching): 为了提高效率，DEX 的订单撮合通常在链下进行。

当前目标 (Current Goal):

我们首先实现原子互换的核心逻辑：创建 HTLC 交易。

步骤 (Steps):

选择交易对 (Choose a Trading Pair): 为了简化，我们仍然选择 BTC/L-BTC (Liquid Bitcoin) 作为交易对。

生成密钥 (Generate Keys): Alice 和 Bob 需要生成 Bitcoin 和 Liquid 的密钥对。

生成密钥 (Generate Keys): Alice 和 Bob 需要生成 Bitcoin 和 Liquid 的密钥对。

创建秘密值 (Create a Secret): Alice 生成一个随机的秘密值 (secret) 并计算其哈希值 (hash)。

构建 HTLC 交易 (Build HTLC Transactions):

Alice 创建一个 HTLC 交易，将 BTC 锁定在一个智能合约中。 这个智能合约要求 Bob 提供 secret 才能解锁 BTC。 如果 Bob 在一定时间内没有提供 secret，Alice 可以取回 BTC。

Bob 创建一个 HTLC 交易，将 L-BTC 锁定在一个智能合约中。 这个智能合约要求 Alice 提供 secret 才能解锁 L-BTC。 如果 Alice 在一定时间内没有提供 secret，Bob 可以取回 L-BTC。

交换哈希值 (Exchange Hashes): Alice 将 hash 发送给 Bob。

执行原子互换 (Execute Atomic Swap):

Bob 提供 secret，解锁 Alice 的 HTLC 交易，获得 BTC。

Alice 提供 secret，解锁 Bob 的 HTLC 交易，获得 L-BTC。