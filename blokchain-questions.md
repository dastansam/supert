# Blockchain Questions

## *(1) Explain some of the ways hashing functions enable blockchain technology *

Hashing functions do not have inverse function (aka one-way function), i.e it is extremely hard to get the input of the hashing function even if you know the output. Hashing functions also are resistant to collision, meaning, there is no two different inputs x and y with the same output z: `x != y, hash(x) != hash(y)`.

In blockchain each block is `chained` to it's predecessor blocks via hashing functions. In fact, blockchain is a linked list of hash pointers. Header hash of the block is produced by hashing function consuming the previous block's header hash and data. This means that if someone wants to manipulate any block in the chain, they would need to change the data of all predecessor blocks until the genesis block, which is extremely hard.

## *(2) briefly explain Bitcoin's UTXO model of transaction validation (separate from POW)*

Each transaction in Bitcoin has number of inputs and outputs. Inputs field contains former outputs that will be consumed and output field is for new creation of coins. Former outputs have the following format: `#tx_id[#tx_out_id]`. In transaction where new Bitcoins are created input field is empty.

UTXO (Unspent TX Output) means that balance of address in Bitcoin is denoted as unspent output. This means that, for example, when Alice has 1 BTC and sends 0.5 BTC to Bob, transaction would look like this:

```
TxId: 2
TxIn: 1[0]
TxOut: 0.5 -> Bob, 0.5 -> Alice
```

Note that 0.5 is being sent to Alice back, it means that now Alice will have `TxOut` of 0.5 i.e her Unspent Tx Output is 0.5

## *(3) what is the structure of a Block in bitcoin and how does it relate to the 'blockchain' (merkle tree vs merkle list of merkle trees)*

Fields of blockchain block:
```md
Block_size: size of the blockchain in bytes
Version: version of the block
Previous_hash: hash of the previous block
Block_hash: hash of the merkle root node of a merkle tree with transactions, calculated from the previous block's fields from version to nonce
Time: timestamp when the block was produced
Difficulty: current difficulty of block mining
Nonce: required for POW
Transaction_counter: number of Txs included in the block
Transactions: list of transactions
```

Merkle tree is basically a binary tree but with hash pointers. It is used in calculating the block hash and it is an easy and fast way of verifying if the data is contained in the tree.

## *(4) what problem/s are POW/POS trying to solve? discuss/compare (byzantine fault tolerance, reaching a single consensus on a p2p network)*

In a distributed network when any node wants to mutate the current state of the network, they propose the change to all the other nodes. We assume that not all of the nodes are honest, some of them may try to propose wrong state mutation. Consensus algorithm is the way of coming to an agreement (henceforth, *consensus*) on one state mutation proposed by a *honest* node.

Bizantine fault tolerance is a feature of consensus algorithm that can withstand the failures of Byzantine General's dilemma.

In simple terms, for each block a random node is selected to create a new block, after node produces a block it is sent to the network for other nodes to verify block data and transactions. 

### Proof-of-Work

Proof of work works by making each node to solve a puzzle. Puzzle is a hashing algorithm where the output is known and nodes need to supply random inputs to hashing algorithm that produces the known output. 

- It requires large amount of tries, therefore not energy efficient
- High investment, energy costs
- Very expensive to attack

### Proof-of-Stake

Each node in the network can be a validator by depositing minimum amount of coins. Each registered node then can propose a new block, but only one node will get selected as the validator using some Verifiable Random Function (VRF).

- Low energy costs
- Requires coins to participate in the network
- Rich nodes get richer
- Relatively cheap to attack but mitigated by slashing

