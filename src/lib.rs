#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;

use alloy_primitives::{aliases::U24, Address, FixedBytes, U256};
use alloy_sol_types::{sol, SolValue};

use stylus_sdk::{crypto::keccak, prelude::*};

sol_interface! {
    interface IERC20 {
        function transferFrom(address from, address to, uint256 value) external returns (bool);
        function transfer(address to, uint256 value) external returns (bool);
    }
}

// Define some persistent storage using the Solidity ABI.
sol_storage! {
    #[entrypoint]
    pub struct FairSwap {
        // Mapping of all pools created within the DEX
        mapping(bytes32 => Pool) pools;
    }

    // A pool is a pair of tokens and a fee which together uniquely identify the pool
    // The struct contains additional data that is used to track the pool's state
    pub struct Pool{
        uint256 token0;
        uint256 token1;
        uint24 fee;
        uint256 liquidity;
        uint256 balance0;
        uint256 balance1;
        mapping(bytes32 => Position) positions;
    }

    // A position is a user's share of the pool's liquidity
    pub struct Position{
        address owner;
        uint256 liquidity;
    }
}

sol! {
    // Thrown when a pool with the same ID already exists
    error PoolAlreadyExists(bytes32 pool_id);
    // Thrown when an action is attempted on a pool that does not exist
    error PoolDoesNotExist(bytes32 pool_id);
    // Thrown when a user attempts to mint liquidity without providing enough tokens
    error InsufficientLiquidityMinted();
    // Thrown when a user attempts to swap with an insufficient amount of tokens
    error InsufficientAmount();
    // Thrown when a user attempts to remove liquidity more than their share of the pool
    error InsufficientLiquidityOwned();
    // Thrown when a token transfer fails
    error FailedOrInsufficientTokenTransfer(address token, address from, address to, uint256 amount);
    // Thrown when the contract fails to refund leftover ETH to the user
    error FailedToReturnExtraEth(address to, uint256 amount);
    // Thrown when the user's swap exceeds their slippage tolerance
    error TooMuchSlippage();

    // Emitted when a pool is created
    event PoolCreated(bytes32 pool_id, address token0, address token1, uint24 fee);
    // Emitted when liquidity is minted
    event LiquidityMinted(bytes32 pool_id, address owner, uint256 liquidity);
    // Emitted when liquidity is burned
    event LiquidityBurned(bytes32 pool_id, address owner, uint256 liquidity);
    // Emitted when a swap is executed
    event Swap(bytes32 pool_id, address user, uint256 input_amount, uint256 output_amount_after_fees, uint256 fees, bool zero_for_one);
}

// Define the Rust-equivalent of the Solidity errors
#[derive(SolidityError)]
pub enum FairSwapError {
    PoolAlreadyExists(PoolAlreadyExists),
    PoolDoesNotExist(PoolDoesNotExist),
    InsufficientAmount(InsufficientAmount),
    InsufficientLiquidityMinted(InsufficientLiquidityMinted),
    InsufficientLiquidityOwned(InsufficientLiquidityOwned),
    FailedOrInsufficientTokenTransfer(FailedOrInsufficientTokenTransfer),
    FailedToReturnExtraEth(FailedToReturnExtraEth),
    TooMuchSlippage(TooMuchSlippage),
}
