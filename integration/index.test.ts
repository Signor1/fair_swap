import { expect, test } from "bun:test";
import { deployMockErc20 } from "./mockErc20";
import {
  addLiquidity,
  createPool,
  fairSwapContractInstance,
  removeLiquidity,
  swap,
} from "./fairSwap";

test("Cannot create pool with same token pair and fee value twice", async () => {
  const tokenOne = await deployMockErc20("Test One", "ONE");
  const tokenTwo = await deployMockErc20("Test Two", "TWO");

  await createPool(tokenOne, tokenTwo, 1000);

  expect(createPool(tokenOne, tokenTwo, 1000)).rejects.toThrow(
    "PoolAlreadyExists",
  );
});

test("Cannot add liquidity or swap in a pool that does not exist", async () => {
  const randomPoolId =
    "0x0000000000000000000000000000000000000000000000000000000000000000";

  expect(
    addLiquidity(randomPoolId, 100_000n, 100_000n, 0n, 0n),
  ).rejects.toThrow("PoolDoesNotExist");

  expect(swap(randomPoolId, 10n, 0n, true)).rejects.toThrow("PoolDoesNotExist");
});

test("Cannot remove more liquidity than you have", async () => {
  const tokenOne = await deployMockErc20("Test One", "ONE");
  const tokenTwo = await deployMockErc20("Test Two", "TWO");

  const [poolId, _token0, _token1] =
    await fairSwapContractInstance.read.getPoolId([tokenOne, tokenTwo, 1000]);

  await createPool(tokenOne, tokenTwo, 1000);
  await addLiquidity(poolId, 100_000n, 100_000n, 0n, 0n);

  expect(removeLiquidity(poolId, 500_000n)).rejects.toThrow(
    "InsufficientLiquidityOwned",
  );
});
