use ethereum_consensus::deneb::mainnet as spec;
use reth_primitives::transaction::{TransactionSigned, TxType};
use ssz_rs::prelude::*;
use std::{env, fs};

fn main() {
    // NOTE: expects a path to a SSZ-encoded signed beacon block
    let args = env::args().collect::<Vec<String>>();
    let path = args.last().unwrap();
    let f = fs::read(path).unwrap();
    let mut block = spec::SignedBeaconBlock::deserialize(&f).unwrap();
    dbg!(block.message.slot);
    dbg!(String::from_utf8(block.message.body.graffiti.to_vec()).unwrap());
    dbg!(String::from_utf8(block.message.body.execution_payload.extra_data.to_vec()).unwrap());
    dbg!(block.message.body.execution_payload.blob_gas_used);
    dbg!(&block.message.body.blob_kzg_commitments);
    for commitment in block.message.body.blob_kzg_commitments.iter() {
        let versioned_hash = spec::kzg_commitment_to_versioned_hash(commitment);
        dbg!(versioned_hash);
    }
    for txn in block.message.body.execution_payload.transactions.iter_mut() {
        let txn = TransactionSigned::decode_enveloped(&mut txn.as_ref()).unwrap();
        if matches!(txn.transaction.tx_type(), TxType::Eip4844) {
            dbg!(txn);
        }
    }
}
