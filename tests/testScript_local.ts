import { ApiPromise, WsProvider } from "@polkadot/api";
import { typesBundle } from "moonbeam-types-bundle";
async function main() {
  const wsProvider = new WsProvider(`ws://localhost:30544`);
  const polkadotApi = await ApiPromise.create({
    provider: wsProvider,
    typesBundle,
  });
  const signedBlock = await polkadotApi.rpc.chain.getBlock();
  console.log("signedBlock", signedBlock.block.header.number.toString());
}
main();