import fs from "fs";
import { ethers } from "ethers";

// -----------------------
// 1️⃣ Configuration
// -----------------------
const CONFIG = {
    proofFilePath: "./SP1Proof.json",
    rpcUrl: "https://testnet.hashio.io/api", // Hedera testnet
    verifierAddress: "", // Your actual contract address
    gasLimit: 3000000,
};

// -----------------------
// 2️⃣ Correct SP1 Verifier ABI
// -----------------------
const SP1_VERIFIER_ABI = [
    {
        "type":"constructor",
        "inputs":[],
        "stateMutability":"nonpayable"
    },
    {
        "type":"function",
        "name":"sp1Verifier",
        "inputs":[],
        "outputs":[{"name":"","type":"address","internalType":"contract ISP1Verifier"}],
        "stateMutability":"view"
    },
    {
        "type":"function",
        "name":"verifySuccinctProof",
        "inputs":[
            {"name":"vk","type":"bytes","internalType":"bytes"},
            {"name":"proof","type":"bytes","internalType":"bytes"},
            {"name":"publicInputs","type":"uint256[]","internalType":"uint256[]"}
        ],
        "outputs":[{"name":"","type":"bool","internalType":"bool"}],
        "stateMutability":"view"
    }
];

// -----------------------
// 3️⃣ Debug and Utility Functions
// -----------------------
async function debugContractCall() {
    try {
        const provider = new ethers.JsonRpcProvider(CONFIG.rpcUrl);
        const verifier = new ethers.Contract(CONFIG.verifierAddress, SP1_VERIFIER_ABI, provider);
        
        console.log("🔍 Getting SP1 Verifier address from contract...");
        const sp1VerifierAddr = await verifier.sp1Verifier();
        console.log("📍 SP1 Verifier Address:", sp1VerifierAddr);
        
        // Check if SP1 verifier contract exists
        const sp1Code = await provider.getCode(sp1VerifierAddr);
        console.log("📋 SP1 Verifier Contract exists:", sp1Code !== '0x');
        
        return sp1VerifierAddr;
    } catch (error) {
        console.error("❌ Failed to get SP1 verifier address:", error.message);
        return null;
    }
}

async function checkContractMethods() {
    try {
        const provider = new ethers.JsonRpcProvider(CONFIG.rpcUrl);
        const verifier = new ethers.Contract(CONFIG.verifierAddress, SP1_VERIFIER_ABI, provider);
        
        console.log("🔍 Testing contract methods...");
        
        // Test if sp1Verifier method exists
        try {
            const sp1Addr = await verifier.sp1Verifier();
            console.log("✅ sp1Verifier() method works:", sp1Addr);
        } catch (e) {
            console.error("❌ sp1Verifier() method failed:", e.message);
        }
        
        return true;
    } catch (error) {
        console.error("❌ Contract method check failed:", error.message);
        return false;
    }
}
function loadProofData(filePath) {
    try {
        if (!fs.existsSync(filePath)) {
            throw new Error(`Proof file not found: ${filePath}`);
        }
        
        const proofData = JSON.parse(fs.readFileSync(filePath, "utf-8"));
        
        const requiredFields = ['proof', 'publicValues', 'vkey'];
        for (const field of requiredFields) {
            if (!proofData[field]) {
                throw new Error(`Missing required field in proof JSON: ${field}`);
            }
        }
        
        return proofData;
    } catch (error) {
        throw new Error(`Failed to load proof data: ${error.message}`);
    }
}

function extractVKey(proofData) {
    if (proofData.vkey) {
        return proofData.vkey;
    }
    throw new Error("Verification key not found in proof data");
}

function formatBytes(data) {
    if (typeof data === 'string') {
        return data.startsWith('0x') ? data : '0x' + data;
    }
    if (Array.isArray(data)) {
        return '0x' + data.map(b => b.toString(16).padStart(2, '0')).join('');
    }
    throw new Error('Invalid data format for bytes conversion');
}

function formatVKey(vkeyHex) {
    // VKey should be bytes, not bytes32
    return formatBytes(vkeyHex);
}

function parsePublicInputs(publicValuesHex) {
    // Convert publicValues hex to uint256 array
    try {
        const hex = publicValuesHex.startsWith('0x') ? publicValuesHex.slice(2) : publicValuesHex;
        
        console.log("🔍 Parsing public inputs from hex...");
        console.log("Original hex length:", hex.length);
        
        // Break into 32-byte chunks and convert to uint256 array
        const chunks = [];
        for (let i = 0; i < hex.length; i += 64) {
            const chunk = hex.slice(i, i + 64);
            if (chunk.length === 64) {
                const value = ethers.getBigInt('0x' + chunk);
                chunks.push(value);
                console.log(`Chunk ${chunks.length - 1}: 0x${chunk} -> ${value}`);
            }
        }
        
        console.log("📊 Public inputs array length:", chunks.length);
        return chunks;
    } catch (error) {
        console.error("Failed to parse public inputs:", error.message);
        
        // Try alternative parsing - maybe the data is structured differently
        console.log("🔄 Trying alternative parsing...");
        try {
            const hex = publicValuesHex.startsWith('0x') ? publicValuesHex.slice(2) : publicValuesHex;
            
            // Maybe it's ABI encoded - try to decode as uint256[]
            const decoded = ethers.AbiCoder.defaultAbiCoder().decode(
                ['uint256[]'], 
                '0x' + hex
            );
            
            console.log("✅ Alternative parsing successful:", decoded[0].length, "elements");
            return decoded[0];
            
        } catch (altError) {
            console.error("Alternative parsing also failed:", altError.message);
            throw error;
        }
    }
}

function analyzePublicValues(publicValuesHex) {
    const hex = publicValuesHex.startsWith('0x') ? publicValuesHex.slice(2) : publicValuesHex;
    
    console.log("🔬 Analyzing public values structure...");
    console.log("Total length:", hex.length, "characters", "(", hex.length/2, "bytes)");
    
    // Break into 64-character chunks (32 bytes each)
    const chunks = [];
    for (let i = 0; i < hex.length; i += 64) {
        chunks.push(hex.slice(i, i + 64));
    }
    
    console.log("Number of 32-byte chunks:", chunks.length);
    
    chunks.forEach((chunk, index) => {
        console.log(`Chunk ${index}: 0x${chunk}`);
        
        // Try to decode as ASCII
        try {
            let ascii = '';
            for (let i = 0; i < chunk.length; i += 2) {
                const byte = parseInt(chunk.substr(i, 2), 16);
                if (byte >= 32 && byte <= 126) {
                    ascii += String.fromCharCode(byte);
                } else if (byte === 0) {
                    ascii += '·';
                } else {
                    ascii += '?';
                }
            }
            if (ascii.includes('07AAATC') || ascii.includes('CONSUMER') || ascii.trim().length > 5) {
                console.log(`  As ASCII: "${ascii}"`);
            }
        } catch (e) {}
    });
    
    // Look for specific patterns
    const gstPattern = Buffer.from("07AAATC0869P1ZB").toString('hex');
    const namePattern = Buffer.from("CONSUMER UNITY AND TRUST SOCIETY").toString('hex');
    
    const gstIndex = hex.indexOf(gstPattern);
    const nameIndex = hex.indexOf(namePattern);
    
    if (gstIndex >= 0) {
        console.log(`✅ Found GST number at position: ${gstIndex/2} bytes`);
    }
    if (nameIndex >= 0) {
        console.log(`✅ Found legal name at position: ${nameIndex/2} bytes`);
    }
}

function decodeGSTPublicValues(publicValuesHex) {
    try {
        const hex = publicValuesHex.startsWith('0x') ? publicValuesHex.slice(2) : publicValuesHex;
        
        console.log("🔍 Decoding public values...");
        console.log("Raw hex length:", hex.length);
        
        // Manual parsing approach since ABI decoding failed
        const gstHex = Buffer.from("07AAATC0869P1ZB").toString('hex');
        const gstIndex = hex.indexOf(gstHex);
        
        const nameHex = Buffer.from("CONSUMER UNITY AND TRUST SOCIETY").toString('hex'); 
        const nameIndex = hex.indexOf(nameHex);
        
        if (gstIndex >= 0 && nameIndex >= 0) {
            console.log("✅ Successfully parsed GST and name from hex data");
            return {
                gstNumber: "07AAATC0869P1ZB",
                legalName: "CONSUMER UNITY AND TRUST SOCIETY",
                manuallyParsed: true
            };
        }
        
        return null;
    } catch (error) {
        console.warn("⚠️  Could not decode public values:", error.message);
        return null;
    }
}

async function validateConfig() {
    const issues = [];
    
    if (!fs.existsSync(CONFIG.proofFilePath)) {
        issues.push(`Proof file not found: ${CONFIG.proofFilePath}`);
    }
    
    if (!ethers.isAddress(CONFIG.verifierAddress)) {
        issues.push("Invalid verifier address format");
    }
    
    if (issues.length > 0) {
        console.error("❌ Configuration Issues:");
        issues.forEach(issue => console.error("  -", issue));
        return false;
    }
    
    return true;
}

async function getContractInfo() {
    try {
        const provider = new ethers.JsonRpcProvider(CONFIG.rpcUrl);
        
        const code = await provider.getCode(CONFIG.verifierAddress);
        if (code === '0x') {
            throw new Error("No contract found at the specified address");
        }
        
        console.log("📋 Contract Info:");
        console.log("  Address:", CONFIG.verifierAddress);
        console.log("  Code Length:", code.length);
        
        return true;
    } catch (error) {
        console.error("Failed to get contract info:", error.message);
        return false;
    }
}

// -----------------------
// 4️⃣ Main Verification Function
// -----------------------
async function verifyProofOnChain() {
    let provider;
    let verifier;
    
    try {
        console.log("🔍 Loading proof data...");
        const proofData = loadProofData(CONFIG.proofFilePath);
        console.log("✅ Proof data loaded successfully");
        
        // Display the extracted GST data
        console.log("📊 Extracted GST Data:");
        console.log("  GST Number:", proofData.gstNumber);
        console.log("  Legal Name:", proofData.legalName);
        console.log("  Signature Valid:", proofData.signatureValid);
        console.log("  Document Commitment:", proofData.documentCommitment);
        console.log("  Public Key Hash:", proofData.publicKeyHash);
        
        // Setup provider
        console.log("🌐 Connecting to Hedera testnet...");
        provider = new ethers.JsonRpcProvider(CONFIG.rpcUrl);
        
        const network = await provider.getNetwork();
        console.log("✅ Connected to network:", network.name, "Chain ID:", network.chainId);
        
        // Setup contract
        console.log("📜 Connecting to verifier contract...");
        verifier = new ethers.Contract(CONFIG.verifierAddress, SP1_VERIFIER_ABI, provider);
        
        // Prepare verification parameters
        console.log("🔧 Preparing verification parameters...");
        
        const vkey = formatVKey(extractVKey(proofData));
        const proofBytes = formatBytes(proofData.proof);
        const publicInputs = parsePublicInputs(proofData.publicValues);
        
        console.log("📝 Verification parameters:");
        console.log("  VKey:", vkey);
        console.log("  VKey Length:", vkey.length);
        console.log("  Proof Length:", proofBytes.length);
        console.log("  Public Inputs Count:", publicInputs.length);
        
        // Analyze the public values structure
        console.log("🔬 Analyzing public values structure...");
        analyzePublicValues(proofData.publicValues);
        
        const decodedValues = decodeGSTPublicValues(proofData.publicValues);
        if (decodedValues) {
            console.log("🔍 Decoded Public Values:");
            if (decodedValues.gstNumber) console.log("  GST from proof:", decodedValues.gstNumber);
            if (decodedValues.legalName) console.log("  Name from proof:", decodedValues.legalName);
            if (decodedValues.manuallyParsed) console.log("  ℹ️  Data was manually parsed from hex");
        }
        
        // Estimate gas with detailed error info
        console.log("⛽ Estimating gas...");
        try {
            const gasEstimate = await verifier.verifySuccinctProof.estimateGas(
                vkey,
                proofBytes,
                publicInputs
            );
            console.log("⛽ Estimated gas:", gasEstimate.toString());
        } catch (gasError) {
            console.warn("⚠️  Gas estimation failed:", gasError.message);
            console.warn("⚠️  This indicates the transaction will revert");
            
            // Try to get more details about why it's failing
            if (gasError.data) {
                console.log("📄 Error data:", gasError.data);
            }
            
            // Let's try to call it anyway to see what happens
            console.log("🔄 Attempting call anyway to get more error details...");
        }
        
        // Try the call (this might give us better error info)
        console.log("🔐 Attempting proof verification...");
        try {
            const startTime = Date.now();
            
            const result = await verifier.verifySuccinctProof(
                vkey,
                proofBytes,
                publicInputs,
                { gasLimit: CONFIG.gasLimit }
            );
            
            const endTime = Date.now();
            
            console.log("✅ PROOF VERIFICATION RESULT:", result);
            console.log("📊 Verification Details:");
            console.log("  Result:", result ? "VALID ✅" : "INVALID ❌");
            console.log("  Time taken:", (endTime - startTime) / 1000, "seconds");
            
            return result;
            
        } catch (callError) {
            console.error("❌ Verification call failed:");
            console.error("   Message:", callError.message);
            
            if (callError.reason) {
                console.error("   Reason:", callError.reason);
            }
            
            if (callError.data) {
                console.error("   Data:", callError.data);
            }
            
            // Common reasons for failure:
            console.log("\n🔍 Possible reasons for failure:");
            console.log("1. 📋 VKey doesn't match the program that generated the proof");
            console.log("2. 🔑 Proof is malformed or corrupted");
            console.log("3. 📊 Public inputs don't match what the proof was generated for");
            console.log("4. 🌐 Wrong SP1 verifier version (contract vs proof)");
            console.log("5. ⛽ Insufficient gas limit");
            
            throw callError;
        }
        
        return true;
        
    } catch (error) {
        console.error("❌ VERIFICATION FAILED:");
        
        if (error.code === 'CALL_EXCEPTION') {
            console.error("🚫 Smart contract call failed:");
            console.error("   Reason:", error.reason || "Unknown");
            console.error("   This usually means the proof is invalid or parameters are incorrect");
        } else if (error.code === 'NETWORK_ERROR') {
            console.error("🌐 Network connection error:");
            console.error("   Check your RPC URL and internet connection");
        } else if (error.message.includes('file not found')) {
            console.error("📁 File error:");
            console.error("   Make sure the proof file exists at:", CONFIG.proofFilePath);
        } else {
            console.error("💥 Unexpected error:");
            console.error("   Message:", error.message);
            if (error.stack) {
                console.error("   Stack:", error.stack);
            }
        }
        
        return false;
    }
}

// -----------------------
// 5️⃣ Main Execution
// -----------------------
async function main() {
    console.log("🚀 Starting SP1 Proof Verification on Hedera");
    console.log("=" .repeat(50));
    
    // Validate configuration
    if (!(await validateConfig())) {
        process.exit(1);
    }
    
    // Get contract info and debug
    console.log("🔍 Checking contract and debugging...");
    if (!(await getContractInfo())) {
        process.exit(1);
    }
    
    // Debug contract methods
    await checkContractMethods();
    
    // Get SP1 verifier info
    await debugContractCall();
    
    // Verify proof
    const success = await verifyProofOnChain();
    
    console.log("=" .repeat(50));
    console.log(success ? "🎉 VERIFICATION COMPLETED SUCCESSFULLY" : "💥 VERIFICATION FAILED");
    
    process.exit(success ? 0 : 1);
}

// Run if this file is executed directly
if (import.meta.url === `file://${process.argv[1]}`) {
    main().catch(error => {
        console.error("💥 Unhandled error:", error);
        process.exit(1);
    });
}

export { verifyProofOnChain, getContractInfo, validateConfig };