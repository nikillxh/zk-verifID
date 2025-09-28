import { ethers } from "ethers";
import { SP1VerifierABI } from "./SP1VerifierABI.js"; // your ABI file

// 1️⃣ Connect to Hedera EVM (testnet or mainnet)
const RPC_URL = "https://testnet.hashio.io/api"; // replace with mainnet if needed
const provider = new ethers.JsonRpcProvider(RPC_URL);

// 2️⃣ Deployed SP1Verifier contract address
const verifierAddress = "0x4a7d5ab9aa0434fca1cd62735d3fac920da20475"; // replace with actual address
const verifier = new ethers.Contract(verifierAddress, SP1VerifierABI, provider);

// 3️⃣ Your SP1GSTProofFixture JSON
const proofJSON = {
  vk: "0x00bde141fa033dfe8375bc4d2670787ba8765bbb2e4e1e758ab9bc3d5fd83610",
  proof: "0xa4594c590b8d6cfc10401b656617cb5a29a219c055a2aacf3cf71ffb448503bc7783922c2650e8b6efba49acbcfc7b56e7b27fe169de251b7791baca8cc73f108cb802971b8bd74abce872f44ac7c98e39ed3b3c0339fc157dde6e113520d1a859add1361bcc756712de1a65dcd5dbd5c089eccfea557894f7e7584ff37a9c43ca47ad6d0982ac799a760a4f57116b982b9d17d3b1b340f6a94dadffde704bb234abf19d0f49d0d421049e56840f044e686590bc5a9d1b33000d37195c0fab05d931bd79188e5e23849b815408a80cca9d7c041af094b5b6977d425679dfc1a54c559ff217e669f7d56b357d7d5a666a185f7d06efc987ad23368e6532a7ce113e5bb161",
  publicValues: "0x000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000e00000000000000000000000000000000000000000000000000000000000000001142225354e3ca5494e61155a2d456776b666e03be931638ea95c159548db29d5af174c33a4628f49a1106ad829c30415627cf8ea6336ed7411f3c327bc25b64f000000000000000000000000000000000000000000000000000000000000000f303741414154433038363950315a4200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000020434f4e53554d455220554e49545920414e4420545255535420534f4349455459"
};

// 4️⃣ Convert publicValues hex into array of BigInts
function hexToBigIntArray(hex) {
  // Each 32-byte segment is one uint256
  const chunks = hex.match(/.{1,64}/g); // 64 hex chars = 32 bytes
  return chunks.map(c => BigInt("0x" + c));
}

const publicInputs = hexToBigIntArray(proofJSON.publicValues.slice(2)); // remove '0x'

// 5️⃣ Call the view function
async function verify() {
  try {
    const isValid = await verifier.verifySuccinctProof(
      proofJSON.vk,
      proofJSON.proof,
      publicInputs
    );
    console.log("Proof valid?", isValid);
  } catch (err) {
    console.error("Verification error:", err);
  }
}

verify();
