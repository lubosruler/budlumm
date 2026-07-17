//! SocialFi boost dağılımı regresyon mühürleri (F4 — ARENAX raporu bulgusu,
//! ARENA3 test mühürü, 2026-07-17).
//!
//! Constitution §3: NFT boost akışının %4'ü B.U.D. storage operatörlerine,
//! %16'sı content creator'a, %80'i protocol'e (burn/treasury). 5322e00 öncesi
//! %4 hesaplanıp hiçbir hesaba yazılmıyordu (implicit burn). Bu testler:
//! (1) iki operatöre eşit dağıtımı, (2) remainder'ın deterministik olarak ilk
//! operatöre (BTreeMap adres sırası) gittiğini, (3) operatör yoksa dürüst
//! burn fallback'ini kilitler.

use crate::chain::blockchain::Blockchain;
use crate::consensus::pow::PoWEngine;
use crate::core::address::Address;
use crate::core::transaction::{Transaction, TransactionType};
use crate::registry::role::roles;
use crate::storage::content_id::ContentId;
use crate::storage::db::Storage;
use std::sync::Arc;
use tempfile::tempdir;

const BOOST_AMOUNT: u64 = 250; // bud_share = 10, creator_share = 40, protocol = 200

fn fresh_chain(db_path: &str) -> Blockchain {
    let storage = Storage::new(db_path).unwrap();
    let consensus = Arc::new(PoWEngine::new(0));
    let mut bc = Blockchain::new(consensus, Some(storage), 1337, None);
    bc.state.base_fee = 0;
    bc.mempool.set_min_fee(0);
    bc
}

fn mint_nft(bc: &mut Blockchain, owner: Address, cid: ContentId) {
    let data = bincode::serialize(&(cid, None::<String>)).unwrap();
    let mut tx = Transaction::new(owner, Address::zero(), 0, data);
    tx.tx_type = TransactionType::NftMint;
    tx.fee = 1;
    tx.hash = tx.calculate_hash();
    bc.mempool.add_transaction(tx).unwrap();
    bc.produce_block(Address::zero());
}

fn boost_nft(bc: &mut Blockchain, booster: Address, nft_id: u64, amount: u64) {
    let mut tx = Transaction::new(booster, Address::zero(), 0, Vec::new());
    tx.tx_type = TransactionType::NftBoost { nft_id, amount };
    tx.fee = 1;
    tx.hash = tx.calculate_hash();
    bc.mempool.add_transaction(tx).unwrap();
    bc.produce_block(Address::zero());
}

#[tokio::test]
async fn boost_share_splits_between_two_active_operators() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("boost_two_ops.db");
    let mut bc = fresh_chain(db.to_str().unwrap());

    let alice = Address::from([0xAA; 32]);
    let bob = Address::from([0xBB; 32]);
    let op1 = Address::from([0x01; 32]);
    let op2 = Address::from([0x02; 32]);
    bc.state.add_balance(&alice, 1000);
    bc.state.add_balance(&bob, 1_000_000);
    bc.state.registry.register(op1, roles::STORAGE_OPERATOR, 10_000_000, 0).unwrap();
    bc.state.registry.register(op2, roles::STORAGE_OPERATOR, 10_000_000, 0).unwrap();

    mint_nft(&mut bc, alice, ContentId([0x77; 32]));
    assert_eq!(bc.state.get_balance(&alice), 999);
    boost_nft(&mut bc, bob, 0, BOOST_AMOUNT);

    // %16 creator
    assert_eq!(bc.state.get_balance(&alice), 999 + 40);
    // %4 B.U.D. = 10 -> iki operatöre 5'er (remainder 0)
    assert_eq!(bc.state.get_balance(&op1), 5);
    assert_eq!(bc.state.get_balance(&op2), 5);
    // booster: 1_000_000 - 250 (boost) - 1 (fee)
    assert_eq!(bc.state.get_balance(&bob), 999_749);
}

#[tokio::test]
async fn boost_remainder_goes_to_first_operator_deterministically() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("boost_three_ops.db");
    let mut bc = fresh_chain(db.to_str().unwrap());

    let alice = Address::from([0xAA; 32]);
    let bob = Address::from([0xBB; 32]);
    let op1 = Address::from([0x01; 32]);
    let op2 = Address::from([0x02; 32]);
    let op3 = Address::from([0x03; 32]);
    bc.state.add_balance(&alice, 1000);
    bc.state.add_balance(&bob, 1_000_000);
    for op in [op1, op2, op3] {
        bc.state.registry.register(op, roles::STORAGE_OPERATOR, 10_000_000, 0).unwrap();
    }

    mint_nft(&mut bc, alice, ContentId([0x78; 32]));
    boost_nft(&mut bc, bob, 0, BOOST_AMOUNT);

    // bud_share = 10 -> per = 3, remainder = 1.
    // registrations BTreeMap<(RoleId, Address)> deterministik sıralar: ilk
    // operatör en küçük adresli (0x01) -> remainder ONA gider (konsensüs kilidi).
    assert_eq!(bc.state.get_balance(&op1), 4);
    assert_eq!(bc.state.get_balance(&op2), 3);
    assert_eq!(bc.state.get_balance(&op3), 3);
}

#[tokio::test]
async fn boost_without_operators_falls_back_to_honest_burn() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("boost_no_ops.db");
    let mut bc = fresh_chain(db.to_str().unwrap());

    let alice = Address::from([0xAA; 32]);
    let bob = Address::from([0xBB; 32]);
    let ghost = Address::from([0x09; 32]);
    bc.state.add_balance(&alice, 1000);
    bc.state.add_balance(&bob, 1_000_000);

    mint_nft(&mut bc, alice, ContentId([0x79; 32]));
    boost_nft(&mut bc, bob, 0, BOOST_AMOUNT);

    // Operatör yok: creator %16'sını yine alır, %4 + %80 kimseye yazılmaz
    // (dürüst burn fallback — hiçbir operatör hesabı oluşmamalı).
    assert_eq!(bc.state.get_balance(&alice), 999 + 40);
    assert_eq!(bc.state.get_balance(&bob), 999_749);
    assert_eq!(bc.state.get_balance(&ghost), 0);
}
