use sp1_sdk::{include_elf, HashableKey, Prover, ProverClient};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const ZKPDF_TEMPLATE_ELF: &[u8] = include_elf!("zkpdf-template-program");

fn main() {
    let prover = ProverClient::builder().cpu().build();
    let (_, vk) = prover.setup(ZKPDF_TEMPLATE_ELF);
    println!("{}", vk.bytes32());
}
