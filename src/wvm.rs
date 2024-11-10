use borsh::{BorshDeserialize, BorshSerialize};
use brotlic;
use solana_sdk::{
    hash::Hash,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use std::io::Write;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct SolAccountMeta {
    pubkey: Pubkey,
    is_signer: bool,
    is_writable: bool,
}

impl From<AccountMeta> for SolAccountMeta {
    fn from(meta: AccountMeta) -> Self {
        Self {
            pubkey: meta.pubkey,
            is_signer: meta.is_signer,
            is_writable: meta.is_writable,
        }
    }
}

impl From<SolAccountMeta> for AccountMeta {
    fn from(wam: SolAccountMeta) -> Self {
        Self {
            pubkey: wam.pubkey,
            is_signer: wam.is_signer,
            is_writable: wam.is_writable,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct SvmInstruction {
    program_id: Pubkey,
    accounts: Vec<SolAccountMeta>,
    data: Vec<u8>,
}

impl From<Instruction> for SvmInstruction {
    fn from(ix: Instruction) -> Self {
        Self {
            program_id: ix.program_id,
            accounts: ix.accounts.into_iter().map(|meta| meta.into()).collect(),
            data: ix.data,
        }
    }
}

impl From<SvmInstruction> for Instruction {
    fn from(wix: SvmInstruction) -> Self {
        Self {
            program_id: wix.program_id,
            accounts: wix.accounts.into_iter().map(|wam| wam.into()).collect(),
            data: wix.data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct WvmData {
    pub instructions: Vec<SvmInstruction>,
    pub payer: Pubkey,
    pub svm_blockhash: Hash,
}

impl WvmData {
    pub fn from(instructions: &[Instruction], payer: Pubkey, svm_blockhash: Hash) -> Self {
        Self {
            instructions: instructions
                .iter()
                .map(|ix| SvmInstruction::from(ix.clone()))
                .collect(),
            payer,
            svm_blockhash,
        }
    }

    pub fn serialize(data: Self) -> Vec<u8> {
        // Borsh serialize
        let serialized = borsh::to_vec(&data).unwrap();
        // Brotli compress the serialized data
        let mut writer = brotlic::CompressorWriter::new(Vec::new());
        writer.write_all(&serialized).unwrap();
        writer.into_inner().unwrap()
    }
}
