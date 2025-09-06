
# FairSwap DEX

A decentralized exchange (DEX) built on Arbitrum Stylus using Rust. FairSwap implements an automated market maker (AMM) protocol similar to Uniswap V2, allowing users to create liquidity pools, provide liquidity, and swap tokens with minimal fees.

> **⚠️ WARNING: This project is for educational and demonstration purposes only. It is NOT production-ready and should not be used with real funds. A comprehensive security audit would be required before any production deployment.**

## Features

- **Pool Creation**: Create liquidity pools for any ERC-20 token pair with custom fees
- **Liquidity Provision**: Add and remove liquidity from pools to earn trading fees
- **Token Swapping**: Swap between tokens using the constant product formula (x * y = k)
- **ETH Support**: Native ETH trading alongside ERC-20 tokens
- **Fee System**: Configurable swap fees (e.g., 0.1%, 0.3%, 1.0%) paid to liquidity providers
- **Position Tracking**: Track individual liquidity provider positions

## Smart Contract Architecture

The DEX consists of several key components:

- **FairSwap**: Main contract implementing the DEX functionality
- **Pool**: Individual liquidity pools for token pairs
- **Position**: User liquidity positions within pools
- **IERC20**: Interface for ERC-20 token interactions

### Core Functions

```rust
// Create a new liquidity pool
pub fn create_pool(token_a: Address, token_b: Address, fee: U24) -> Result<(), FairSwapError>

// Add liquidity to earn trading fees
pub fn add_liquidity(pool_id: FixedBytes<32>, amount_0_desired: U256, amount_1_desired: U256, amount_0_min: U256, amount_1_min: U256) -> Result<(), FairSwapError>

// Remove liquidity and claim tokens
pub fn remove_liquidity(pool_id: FixedBytes<32>, liquidity_to_remove: U256) -> Result<(), FairSwapError>

// Swap tokens
pub fn swap(pool_id: FixedBytes<32>, input_amount: U256, min_output_amount: U256, zero_for_one: bool) -> Result<(), FairSwapError>
```

## Quick Start

Install [Rust](https://www.rust-lang.org/tools/install), and then install the Stylus CLI tool with Cargo:

```bash
cargo install --force cargo-stylus cargo-stylus-check
```

Add the `wasm32-unknown-unknown` build target to your Rust compiler:

```
rustup target add wasm32-unknown-unknown
```

You should now have it available as a Cargo subcommand:

```bash
cargo stylus --help
```

Then, clone this repository:

```
git clone <repository-url> && cd dex
```

### Testnet Information

All testnet information, including faucets and RPC endpoints can be found [here](https://docs.arbitrum.io/stylus/reference/testnet-information).

### ABI Export

You can export the Solidity ABI for your program by using the `cargo stylus` tool:

```bash
cargo stylus export-abi
```

This outputs the ABI interface that can be used with any Ethereum tooling to interact with the deployed contract.

## Testing

Run the integration tests using Bun:

```bash
cd integration
bun test
```

The tests cover:
- Pool creation and validation
- Liquidity addition and removal
- Token swapping with fees
- ETH/ERC-20 interactions
- Error handling for edge cases

## Deploying

You can use the `cargo stylus` command to deploy your program to the Stylus testnet. First, check that your program compiles correctly:

```bash
cargo stylus check
```

If successful, you should see:

```bash
Finished release [optimized] target(s) in 1.88s
Reading WASM file at dex/target/wasm32-unknown-unknown/release/dex.wasm
Compressed WASM size: X.X KB
Program succeeded Stylus onchain activation checks with Stylus version: 1
```

Next, estimate the gas costs:

```bash
cargo stylus deploy \
  --private-key=<PRIVKEY_KEY> \
  --estimate-gas
```

Finally, deploy:

```bash
cargo stylus deploy \
  --private-key=<PRIVKEY_KEY>
```

## Usage Examples

### Creating a Pool

```typescript
import { createPool } from './integration/fairSwap';

// Create a pool with 1% fee (1000 basis points)
await createPool(tokenA, tokenB, 1000);
```

### Adding Liquidity

```typescript
import { addLiquidity } from './integration/fairSwap';

// Add 100,000 of each token as liquidity
await addLiquidity(poolId, 100_000n, 100_000n, 0n, 0n);
```

### Swapping Tokens

```typescript
import { swap } from './integration/fairSwap';

// Swap 10 tokens of token0 for token1
await swap(poolId, 10n, 0n, true);
```

## Mathematical Formula

FairSwap uses the constant product formula:

```
x * y = k
```

Where:
- `x` = balance of token0 in the pool
- `y` = balance of token1 in the pool
- `k` = constant that must remain unchanged after swaps

Swap fees are deducted from the output amount, providing yield to liquidity providers.
