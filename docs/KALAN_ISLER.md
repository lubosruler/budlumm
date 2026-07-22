# Kalan İşler — Budlum

**Güncelleme:** 2026-07-22 (ARENA2) · Teknik kapanmamış / kısmen kapanmış işler.

---

## AI execution layer

Zincir-üzeri AI **execution** (modelin BudZKVM guest olarak koşması + STARK kanıt)
hâlâ araştırma. Lubot attestation / AiRegistry / soft-incentive **mevcut**.

- İskelet: `docs/AI_ONCHAIN_EXECUTION_RESEARCH.md` (ARENA2).
- VerifyInference (0x1F) execution stub (rd=0); AIR/proof envelope binding açık.
- **Kapanış kriteri:** bounded model-class RFC + mini MLP prove/verify + 0x1F AIR.

## Z-B: BudZKVM VerifyMerkle 64-depth soundness

✅ **KAPANDI (kod + test):** `proves_verify_merkle_valid_64_depth` ve
`task4_diagnose_verify_merkle_matrix_chain` yeşil. Opcode ISA'da experimental
değil; **mainnet kapısı** `MainnetActivation::verify_merkle_enabled` (default
off, ceremony sonrası açılır). Proof-of-storage ürün iddiası hâlâ B.U.D.
entegrasyonuna bağlı — L1 primitive hazır.

## BLS/PQ HSM vendor-native

Kısmen kapandı:

- ✅ Ed25519 PKCS#11 native
- ✅ Vendor mechanism parse + `CKM_VENDOR_DEFINED` altı fail-closed
  (`Pkcs11Signer::validate_vendor_mechanism_id`)
- ✅ `Pkcs11VendorCapabilities` (vendor vs software-fallback advertisement)
- ⏳ Gerçek vendor HSM'de non-extractable BLS/Dilithium keygen + harici audit
  (donanım + Ayaz tedarik; kod ajanı tek başına kapatamaz)

## Gizlilik katmanı — AIR constraint'leri

✅ **KAPANDI (D2 Görev D):**

- Selector kolonları 370–372 (`COL_IS_PRIVACY_*`)
- Paylaşılan Poseidon gadget: Poseidon / PrivacyCommit (3-absorb) / NullifierCheck
- SumConservation equality gate (amount witness; commitment satırlarıyla bağ)
- VM gerçek semantik + gas=10 + DOMAIN_NULLIFIER
- Prove/verify: `d2_proves_privacy_commit`, `d2_proves_nullifier_*`,
  `d2_proves_sum_conservation`, `d2_proves_private_transfer_e2e`

## Gizlilik katmanı — E2E test

✅ **KAPANDI (D2 Görev F):** commit → nullifier → sum-conservation STARK
round-trip (`d2_proves_private_transfer_e2e`). NoteRegistry field-element
köprüsü + cüzdan view-key / note-privacy toggle tamam.

---

*Mainnet kapılarının (MainnetActivation, HSM policy) ceremony/operasyon tarafı
Ayaz + donanım; bu dosya yalnızca teknik kod kapanışını izler.*
