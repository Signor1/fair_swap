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
        address token0;
        address token1;
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

impl FairSwap {
    // impl for private functions
    fn get_pool_id(
        &self,
        token_a: Address,
        token_b: Address,
        fee: U24,
    ) -> (FixedBytes<32>, Address, Address) {
        let token0: Address;
        let token1: Address;

        // Sort the tokens to ensure determinism
        if token_a <= token_b {
            token0 = token_a;
            token1 = token_b;
        } else {
            token0 = token_b;
            token1 = token_a;
        }

        let hash_data = (token0, token1, fee);
        let pool_id = keccak(hash_data.abi_encode_sequence());

        (pool_id, token0, token1)
    }

    fn integer_sqrt(&self, x: U256) -> U256 {
        let two = U256::from(2);

        let mut z: U256 = (x + U256::from(1)) >> 1;
        let mut y = x;

        while z < y {
            y = z;
            z = (x / z + z) / two;
        }

        y
    }
}

#[public]
impl FairSwap {
    pub fn create_pool(
        &mut self,
        token_a: Address,
        token_b: Address,
        fee: U24,
    ) -> Result<(), FairSwapError> {
        let (pool_id, token0, token1) = self.get_pool_id(token_a, token_b, fee);
        let existing_pool = self.pools.get(pool_id);

        // If one of the token addresses of this pool in the mapping is non-zero, the pool already exists in our mapping
        if !existing_pool.token0.get().is_zero() || !existing_pool.token1.get().is_zero() {
            return Err(FairSwapError::PoolAlreadyExists(PoolAlreadyExists {
                pool_id: pool_id,
            }));
        }

        let mut pool_setter = self.pools.setter(pool_id);
        pool_setter.token0.set(token0);
        pool_setter.token1.set(token1);
        pool_setter.fee.set(fee);

        // Initially the pool has no liquidity or token balances
        pool_setter.liquidity.set(U256::from(0));
        pool_setter.balance0.set(U256::from(0));
        pool_setter.balance1.set(U256::from(0));

        // Emit the PoolCreated event
        log(
            self.vm(),
            PoolCreated {
                pool_id,
                token0,
                token1,
                fee,
            },
        );

        Ok(())
    }

    #[payable]
    pub fn add_liquidity(
        &mut self,
        pool_id: FixedBytes<32>,
        amount_0_desired: U256,
        amount_1_desired: U256,
        amount_0_min: U256,
        amount_1_min: U256,
    ) -> Result<(), FairSwapError> {
        todo!();
    }

    pub fn remove_liquidity(
        &mut self,
        pool_id: FixedBytes<32>,
        liquidity_to_remove: U256,
    ) -> Result<(), FairSwapError> {
        todo!();
    }

    #[payable]
    pub fn swap(
        &mut self,
        pool_id: FixedBytes<32>,
        input_amount: U256,
        min_output_amount: U256,
        zero_per_one: bool,
    ) -> Result<(), FairSwapError> {
        todo!();
    }

    // Given a pool ID and an owner address, compute a deterministic Position ID and returns it
    pub fn get_position_id(&self, pool_id: FixedBytes<32>, owner: Address) -> FixedBytes<32> {
        let hash_data = (pool_id, owner);
        let position_id = keccak(hash_data.abi_encode_sequence());
        position_id
    }

    // Given a pool ID and an owner address, return the user's position liquidity
    pub fn get_position_liquidity(&self, pool_id: FixedBytes<32>, owner: Address) -> U256 {
        let position_id = self.get_position_id(pool_id, owner);
        let pool = self.pools.get(pool_id);
        let position = pool.positions.get(position_id);
        position.liquidity.get()
    }
}
