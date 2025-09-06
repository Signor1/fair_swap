import { getContract } from "viem";
import { FairSwapABI } from "./abis";
import { walletClient } from "./chain";

export const FAIR_SWAP_ADDRESS = "0x525c2aba45f66987217323e8a05ea400c65d06dc";

export const fairSwapContractInstancee = getContract({
  abi: FairSwapABI,
  address: FAIR_SWAP_ADDRESS,
  client: walletClient,
});
