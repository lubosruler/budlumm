# Kalan İşler — Budlum

**Güncelleme:** 2026-07-22 · Bu dosya canlı kalan-iş kaynağıdır (faz diline yer yok).

---

## 🔴 Mainnet blokerleri (lansmanı engelleyen — operasyonel)

| # | İş | Kim | Durum |
|---|---|---|---|
| 1 | Bağımsız harici güvenlik audit'i | Ayaz (audit firması) | Checklist hazır (`AUDIT_CHECKLIST.md`), gerçek audit yapılmadı |
| 2 | HSM ceremony (YubiHSM 2 + rehearsal) | Ayaz (donanım ~$650/adet) | Policy/tooling hazır, gerçek donanım + tatbikat yok |
| 3 | 7-gün CI stabilite penceresi | zaman (günlük yeşil takip) | Pencere sayımı sürüyor |
| 4 | Production runbook canlı tatbikatı | Ayaz (host) | Doc-only rehearsal hazır, gerçek backup/restore+halt tatbikatı yok |

## 🟡 Kritik kod/teknik boşluklar

| # | İş | Detay |
|---|---|---|
| 5 | Z-B: BudZKVM VerifyMerkle 64-depth soundness | Production ISA'da gate'li. 64-derinlik kanıt yok → "proof-of-storage" iddiası yapılamıyor |
| 6 | BLS/PQ HSM vendor-native | Ed25519 PKCS#11 var; BLS/Dilithium sadece mock backend |
| 7 | Gizlilik katmanı AIR constraint'leri | Opcode iskeleti (0x20-0x22) + note registry + TEE toggle var; **AIR constraint + gerçek TEE backend yok** |
| 8 | Gizlilik katmanı E2E test | commit→nullifier→sum-conservation uçtan uca test (AIR sonrası) |
| 9 | TEE wallet entegrasyonu | WalletPrivacyConfig toggle var; Wallet struct'ına bağlı değil + gerçek SGX/Nitro backend yok |

## ⚪ Araştırma (v1'de yok, v2+)

| # | İş | Durum |
|---|---|---|
| 10 | AI execution layer (zincir-üzeri AI çalıştırma) | Lubot inference var, on-chain execution tasarlanmadı |
| 11 | Formal verification (TLA+/Coq) | Başlanmadı |
| 12 | ZK privacy layer (tam) | #7-9 ile bağlantılı |

## 📋 Kalite / temizlik

| # | İş | Detay |
|---|---|---|
| 13 | Kodda Phase ifadeleri | ARENA2 docs'i yapıyor; **test adları** (`test_phase*` ~95) + **CI job adları** (`(Phase X.Y)`) hâlâ Phase içeriyor — gate'ler grep ediyor, koordine yeniden adlandırma gerek |
| 14 | Coverage gate | Coverage badge `%` değil ("nextest+llvm-cov") — gerçek % + ratchet |
| 15 | budlum-core repo | Supply-chain gate'leri eklendi — yeşil teyit edilmeli |
| 16 | Ayrı repolar sync (BudZero/B.U.D./Lubot) | Repolar ayrı; budlum ana repo ile içerik uyumu (drift) takip |

---

## Öncelik

- **Hemen (kod):** #7-9 (gizlilik katmanı tamamlama), #13 (kod Phase temizliği)
- **Ayaz bekleyen:** #1-4 (audit/HSM/stability/drill)
- **Araştırma:** #10-12 (v2+)
