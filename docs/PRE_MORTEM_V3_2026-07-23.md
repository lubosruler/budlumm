# 🔴 PRE-MORTEM GÜVENLİK ANALİZİ V3 — 2026-07-23

**Repo:** budlum-xyz/budlum @ `2b8782c` (PR #141 merged)
**Analiz:** ARENA2 — sıfırdan kapsamlı denetim
**Önceki:** V2 (15 bulgu, 15/15 kapatıldı)

---

## ÖZET

| Ciddiyet | Sayı | Durum |
|----------|------|-------|
| 🔴 Kritik | 3 | Yeni bulgu |
| 🟡 Yüksek | 5 | Yeni bulgu |
| ⚪ Düşük | 4 | Yeni bulgu |
| **Toplam** | **12** | **12 açık** |

---

## 🔴 KRİTİK BULGULAR

### K1: HubRegister — Balance Check Eksik (Fon Kaybı)

**Dosya:** `src/execution/executor.rs:750`
**Kod:**
```rust
sender.balance = sender
    .balance
    .saturating_sub(tx.fee)
    .saturating_sub(crate::hub::HUB_REGISTER_MIN_FEE);
```
**Sorun:** `sender.balance >= tx.fee + HUB_REGISTER_MIN_FEE` kontrolü YOK.
`tx.amount >= HUB_REGISTER_MIN_FEE` var ama sender balance check yok.
Balance yetersizse `saturating_sub` sessizce 0'a düşürür — fee ve registration fee
havaya gider, sender 0 bakiye kalır.

**Düzeltme:** Fee deduction'dan önce balance check ekle:
```rust
let total_cost = tx.fee.saturating_add(crate::hub::HUB_REGISTER_MIN_FEE);
if sender.balance < total_cost {
    return Err(BudlumError::validation("insufficient_funds", "..."));
}
```

---

### K2: AiInferenceRequest — Balance Check Belirsiz (Fon Kaybı)

**Dosya:** `src/execution/executor.rs:800`
**Kod:**
```rust
sender.balance = sender
    .balance
    .saturating_sub(tx.fee)
    .saturating_sub(req.max_fee);
```
**Sorun:** `sender.balance >= tx.fee + req.max_fee` kontrolü executor-level'da
görünmüyor. `submit_request` içinde olabilir ama defense-in-depth eksik.
Balance yetersizse `saturating_sub` sessizce 0'a düşürür — max_fee escrow
havaya gider.

**Düzeltme:** Fee deduction'dan önce balance check ekle:
```rust
let total_cost = tx.fee.saturating_add(req.max_fee);
if sender.balance < total_cost {
    return Err(BudlumError::validation("insufficient_funds", "..."));
}
```

---

### K3: Bridge Unlock — `add_balance` instead of `try_add_balance` (Sessiz Overflow)

**Dosya:** `src/execution/executor.rs:667`
**Kod:**
```rust
state.add_balance(&transfer.owner, final_amount as u64);
```
**Sorun:** `add_balance` içerde `saturating_add` kullanıyor. `final_amount > u64::MAX`
check var ama `add_balance` içinde overflow olursa sessizce `u64::MAX`'a cap eder.
Bridge mint path'inde `try_add_balance` kullanılıyor ama unlock path'inde `add_balance`.

**Düzeltme:** `add_balance` → `try_add_balance`:
```rust
state.try_add_balance(&transfer.owner, final_amount as u64)
    .map_err(|e| BudlumError::validation("bridge_unlock_overflow", &e))?;
```

---

## 🟡 YÜKSEK BULGULAR

### H1: NftBoost — `saturating_mul` Overflow (Rounding Leak)

**Dosya:** `src/execution/executor.rs:424-430`
**Kod:**
```rust
let bud_share = amount.saturating_mul(4) / 100;
let creator_share = amount.saturating_mul(16) / 100;
let protocol_share = amount.saturating_sub(bud_share).saturating_sub(creator_share);
```
**Sorun:** `amount > u64::MAX / 16` ise `saturating_mul(16)` = `u64::MAX`.
`creator_share` = `u64::MAX / 100` ≈ 1.8×10¹⁷. `protocol_share` = `amount - bud_share - u64::MAX/100`
= negative → `saturating_sub` = 0. Protocol share havaya gider.

**Düzeltme:** `amount` üst sınır ekle:
```rust
if amount > u64::MAX / 100 {
    return Err(BudlumError::validation("boost_amount_too_large", "..."));
}
```

---

### H2: AI Execution Proof — `program_hash` None Bypass

**Dosya:** `src/ai/execution/verify.rs:48`
**Kod:**
```rust
None => true, // no registered hash → don't fail structural on bind
```
**Sorun:** Model `execution_program_hash` register etmemişse, herhangi bir
`program_hash` kabul ediliyor. Malicious prover herhangi bir program çalıştırıp
proof sunabilir — model owner'ın intended program'ı değil.

**Düzeltme:** `require_execution_proof = true` ise `program_hash` zorunlu:
```rust
None => !model.require_execution_proof,
```

---

### H3: Privacy Note Registry — Sınırsız Büyüme (DoS)

**Dosya:** `src/privacy/note_registry.rs:12-13`
**Kod:**
```rust
live_commitments: BTreeSet<NoteHash>,
spent_nullifiers: BTreeSet<NoteHash>,
```
**Sorun:** Her private transfer 2 commitment ekler, 1 nullifier ekler, 1 commitment
siler. Net +1 commitment + 1 nullifier per transfer. 1M transfer = 2M set entry.
`state_root()` tüm set'leri iterate eder — O(N) latency.

**Düzeltme:** Pruning mekanizması ekle:
- `spent_nullifiers`: epoch-based pruning (N epoch sonra sil)
- `live_commitments`: Merkle tree'ye geçiş (incremental root)

---

### H4: PoS Slashing Evidence — Sınırsız Büyüme (DoS)

**Dosya:** `src/consensus/pos.rs:77`
**Kod:**
```rust
slashing_evidence: RwLock<Vec<SlashingEvidence>>,
```
**Sorun:** Her slashing evidence eklenir, hiç silinmez. Uzun süreli node'da
sınırsız bellek kullanımı. `get_slashing_evidence()` tüm vec'i clone eder.

**Düzeltme:** Epoch-based pruning:
```rust
pub fn prune_evidence(&mut self, before_epoch: u64) {
    self.slashing_evidence.retain(|e| e.epoch >= before_epoch);
}
```

---

### H5: Poseidon Constants — bud-vm Lock Test Eksik

**Dosya:** `budzero/bud-vm/src/lib.rs:1041-1052`
**Sorun:** `wallet-core`'da S4 lock test var ama `bud-vm`'de yok. Birisi
`bud-vm`'deki MDS/RC'yi değiştirirse `wallet-core` test'i geçmez ama
`bud-vm` test'i geçebilir — CI'da tespit edilmeyebilir.

**Düzeltme:** `bud-vm`'e de lock test ekle:
```rust
#[test]
fn poseidon_mds_rc_lock() {
    assert_eq!(MDS[0], [7, 1, 3, 8, 8, 3, 4, 9]);
    assert_eq!(RC[0][0], 0xdd5743e7f2a5a5d9);
}
```

---

## ⚪ DÜŞÜK BULGULAR

### L1: Executor — 103 `saturating` Kullanımı (Sistemik)

**Dosya:** `src/execution/executor.rs`
**Sorun:** 103 `saturating_sub/add` vs 11 `checked_sub/add`. Fee deduction
path'lerinde balance check varsa güvenli ama defense-in-depth eksik.

**Durum:** Kademeli geçiş — kritik path'ler checked'a çevrildi, kalanlar
saturating (balance check ile korunuyor).

---

### L2: Syscall imm=6 — rd_val_new Constraint Eksik

**Dosya:** `budzero/bud-proof/src/plonky3_air.rs`
**Sorun:** S6 fix imm ∉ {1,2,3,6} için `rd_val_new = 0` constraint'i ekledi.
Ama imm=6 (AI event) için `rd_val_new` constraint yok — herhangi bir değer
dönebilir.

**Durum:** imm=6'nın return değeri kullanılmıyor (event push side-effect).
Risk düşük.

---

### L3: Privacy — Commitment-Nullifier Binding Registry Level'da Yok

**Dosya:** `src/privacy/note_registry.rs:59-95`
**Sorun:** `apply_transfer` commitment-nullifier binding doğrulamıyor.
Nullifier gerçekten commitment'tan türetilmiş mi? VM/AIR level'da
NullifierCheck opcode ile doğrulanıyor ama registry level'da değil.

**Durum:** Defense-in-depth eksikliği. VM/AIR constraint yeterli ama
registry level'da da check eklenmeli.

---

### L4: Governance — Proposal Cancellation Owner-Only Ama Epoch Check Yok

**Dosya:** `src/core/governance.rs`
**Sorun:** Proposal cancellation owner-only ama cancellation sonrası
re-submission için cooldown yok. Owner sürekli proposal submit/cancel
yaparak governance spam yapabilir.

**Durum:** Düşük risk — governance spam fee maliyeti var.

---

## KAPATILMIŞ BULGULAR (V2 → V3)

| # | Bulgu | Durum |
|---|-------|-------|
| R2 | Bridge RPC auth (5 method) | ✅ Kapatıldı |
| S1 | PrivacyCommit blinding u32 truncation | ✅ Kapatıldı |
| S2 | SumConservation field-safe comparison | ✅ Kapatıldı |
| S4 | Poseidon constants lock test | ✅ Kapatıldı (wallet-core) |
| S5 | VerifyMerkle env var gate | ✅ Kapatıldı |
| S6 | Syscall AIR constraint | ✅ Kapatıldı |
| B2 | Bridge mint payload_hash verification | ✅ Kapatıldı |
| B3 | ReplayNonceStore pruning | ✅ Kapatıldı |
| E1 | Balance overflow checked (kritik path'ler) | ✅ Kapatıldı (kısmi) |
| W1 | Wallet seed zeroize | ✅ Kapatıldı |
| C1 | BLS hash_to_g1 uniform distribution | ✅ Kapatıldı |
| C3 | Validator consensus key readiness | ✅ Kapatıldı |
| V98 | PoS seed lock poisoning | ✅ Kapatıldı |
| V110 | VerifyInference disabled on mainnet | ✅ Kapatıldı |
| V134 | Relayer fee credit on unlock | ✅ Kapatıldı |

---

## DÜZELTME PLANI — Öncelik Sırası

### Katman 0 — HEMEN (Kritik)

| # | Bulgu | Action | Efor |
|---|-------|--------|------|
| K1 | HubRegister balance check | `if sender.balance < total_cost` ekle | 5 dk |
| K2 | AiInferenceRequest balance check | `if sender.balance < total_cost` ekle | 5 dk |
| K3 | Bridge unlock `try_add_balance` | `add_balance` → `try_add_balance` | 5 dk |

### Katman 1 — MAİNNET ÖNCESİ (Yüksek)

| # | Bulgu | Action | Efor |
|---|-------|--------|------|
| H1 | NftBoost overflow | `amount` üst sınır ekle | 10 dk |
| H2 | AI program_hash None bypass | `require_execution_proof` check ekle | 10 dk |
| H3 | Note registry DoS | Epoch-based pruning ekle | 30 dk |
| H4 | Slashing evidence DoS | Epoch-based pruning ekle | 15 dk |
| H5 | bud-vm Poseidon lock test | Test ekle | 5 dk |

### Katman 2 — MAİNNET SONRASI (Düşük)

| # | Bulgu | Action | Efor |
|---|-------|--------|------|
| L1 | Executor saturating → checked | Kademeli geçiş | 2 saat |
| L2 | Syscall imm=6 constraint | AIR constraint ekle | 30 dk |
| L3 | Commitment-nullifier binding | Registry-level check ekle | 20 dk |
| L4 | Governance spam cooldown | Cooldown ekle | 10 dk |

---

**Bu rapor `2b8782c` commit'i üzerinde yapılmıştır. 12 bulgu tespit edildi:
3 kritik, 5 yüksek, 4 düşük. Katman 0 düzeltmeleri 15 dakikada uygulanabilir.**
