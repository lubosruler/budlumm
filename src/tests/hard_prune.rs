//! Hard Pruning zincir-seviyesi mühür (F1 — ARENAX raporu bulgusu, ARENA3
//! test mühürü, 2026-07-17).
//!
//! Constitution §1: NFT yakılınca bağlı olduğu B.U.D. içeriği silinmelidir.
//! 5322e00'un produce_block yolundaki consensus-seviye prune'unu kilitler:
//! NftBurn içeren blok üretildiğinde, NFT'nin content_id'sine denk gelen
//! storage manifest'i registry'den kalkmalı. Fiziksel chunk silme
//! (NodeCommand::StoragePrune worker) ayrı doğrulama konusudur.

use crate::chain::blockchain::Blockchain;
use crate::consensus::pow::PoWEngine;
use crate::core::address::Address;
use crate::core::transaction::{Transaction, TransactionType};
use crate::storage::manifest::ContentManifest;
use crate::storage::db::Storage;
use std::sync::Arc;
use tempfile::tempdir;

#[tokio::test]
async fn nft_burn_prunes_matching_storage_manifest_on_produce() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("hard_prune_produce.db");
    let storage = Storage::new(db.to_str().unwrap()).unwrap();
    let consensus = Arc::new(PoWEngine::new(0));
    let mut bc = Blockchain::new(consensus, Some(storage), 1337, None);
    bc.state.base_fee = 0;
    bc.mempool.set_min_fee(0);

    let alice = Address::from([0xAA; 32]);
    bc.state.add_balance(&alice, 1000);

    // Manifest zincir registry'sine kayıtlı; NFT aynı content_id'ye bağlı.
    let manifest = ContentManifest::from_bytes_sliced(b"hard prune target", 4).unwrap();
    let cid = manifest.manifest_id;
    bc.storage_registry.register_manifest(&manifest);
    assert!(bc.storage_registry.get_manifest(&cid).is_some());

    let data = bincode::serialize(&(cid, None::<String>)).unwrap();
    let mut mint_tx = Transaction::new(alice, Address::zero(), 0, data);
    mint_tx.tx_type = TransactionType::NftMint;
    mint_tx.fee = 1;
    mint_tx.hash = mint_tx.calculate_hash();
    bc.mempool.add_transaction(mint_tx).unwrap();
    bc.produce_block(Address::zero());
    assert_eq!(bc.state.nft_registry.nfts.len(), 1);

    let burn_data = bincode::serialize(&0u64).unwrap();
    let mut burn_tx = Transaction::new(alice, Address::zero(), 0, burn_data);
    burn_tx.tx_type = TransactionType::NftBurn;
    burn_tx.fee = 1;
    burn_tx.hash = burn_tx.calculate_hash();
    bc.mempool.add_transaction(burn_tx).unwrap();
    bc.produce_block(Address::zero());

    // NFT yakıldı ve eşleşen manifest hard-prune ile silindi.
    assert_eq!(bc.state.nft_registry.nfts.len(), 0);
    assert!(bc.storage_registry.get_manifest(&cid).is_none());
}
