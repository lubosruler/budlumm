# Görev C — Legacy Declared-Depth Proof Kaldırma: Doğrulama Raporu (ARENA1)

**Tarih:** 2026-07-23 · **Ajan:** ARENA1 · **Görev:** C (D3)

## 0. Öncelikli Soru: "Z-B VerifyMerkle 64-depth" == D3 legacy mi?

**SONUÇ: FARKLI (ayrı kalem).** Kod ile doğrulandı, tahmin yok.

- **"Z-B VerifyMerkle 64-depth"** = commit `0e18c35` (#123) *"feat(d2): privacy AIR constraints + E2E prove + view-key + **Z-B closeout**"*. Commit mesajı: *"Z-B — VerifyMerkle 64-depth prove already green; docs/gates updated"*. Bu, **BudZKVM (ZKVM) storage Merkle proof**'udur (`bud-proof`, `bud-vm`, `bud-state`) — D2 gizlilik katmanı kapsamında, yeni **bounded** Merkle kanıtının TAMAMLANMASIDIR (ekleme), kaldırma değildir.
- **D3 "legacy declared-depth proof"** = `src/domain/finality_adapter.rs`'deki **self-declared PoW finality**: `FinalityProof::PoW { declared_head_hash, declared_cumulative_work }`. Bu, cross-domain bridge **finality** yoludur (production ISA), ZKVM Merkle proof'undan tamamen farklı bir alt sistem.

İkisi aynı şey DEĞİL. Kullanıcının "Z-B VerifyMerkle 64-depth kapatması D3 ile aynı mı?" hipotezi kod ile çürütülmüştür: Z-B, D3'ün kaldırdığı legacy yolun yerine KONAN yeni bounded proof'dur (storage/privacy), D3'ün kaldırdığı legacy ise self-declared PoW finality'dir.

## 1. D3 Kararı Gerçekten Uygulandı mı? (kod ile)

Evet — `src/domain/finality_adapter.rs`:

- `FinalityProof::PoW` varyantı yorumu (satır 85): *"Legacy self-declared confirmation proof. It remains decodable for historical domains, but bridge mint never accepts it."*
- `PoWFinalityAdapter::verify_finality` (satır 246-264): D3 (2026-07-22) ile **her zaman `Rejected`** döner: *"legacy self-declared PoW finality retired (D3); use pow-header-chain-v1"*.
- `blockchain.rs` (verify_domain_commitment_finality, PoW dalı satır 904): `domain.finality_adapter == POW_HEADER_CHAIN_ADAPTER` ise `PoWHeaderChainFinalityAdapter` kullanılır; değilse `PoWFinalityAdapter` (yalnızca arşiv, mint yetkisi YOK).
- Test `pow_finality_legacy_adapter_always_rejects_after_d3` (satır 1024): *"D3 (2026-07-22): legacy self-declared PoW adapter NEVER finalizes."* — negatif test mevcut.

Yani D3'ün **gerçek** legacy yolunun fonksiyonel kaldırması main'de TAMAMLANMIŞ: self-declared PoW artık hiçbir şeyi finalize edemez, bridge mint yalnızca `PoWHeaderChain` (yeni bounded header-chain proof) ile çalışır.

## 2. Bağımlılık Taraması (legacy `FinalityProof::PoW` referansları)

Kaldırma KARARI VERİLMEDEN tarama yapıldı (doğrulama olmadan silme commit'i atılmadı):

| Konum | Referans | Durum |
|-------|----------|-------|
| `finality_adapter.rs:85-103` | `FinalityProof::PoW` varyantı + `declared_head_hash`/`declared_cumulative_work` | Tip korunuyor (bincode stabilitesi) |
| `finality_adapter.rs:198-264` | `PoWFinalityAdapter` (her zaman reject) | Hâlâ tanımlı |
| `finality_adapter.rs:1024-1052` | test `pow_finality_legacy_adapter_always_rejects_after_d3` → `FinalityProof::PoW {...}` kurar | Negatif test |
| `plugin.rs:79,86,93` | PoW domain'lerin default finality adapter'ı = `PoWFinalityAdapter` | Hâlâ bağlı |
| `main.rs:684` | `ConsensusType::PoW → ("pow-confirmation-depth", 64)` | Hâlâ bağlı |
| `blockchain.rs:904-912` | PoW dalı historical-compat için `PoWFinalityAdapter::default()` | Hâlâ bağlı |
| `domain/registry.rs`, `commitment_registry.rs`, `fork_choice.rs`, `types.rs`, `genesis.rs`, `network/proto_conversions.rs` | `ConsensusKind::PoW` (consensus KIND, ayrı kavram) | Meşru, korunmalı |

**Tespit:** Legacy `FinalityProof::PoW` varyantı, **serileştirme geriye-dönük uyumu (bincode variant stabilitesi)** ve **historical-compat dalı** nedeniyle TİP olarak korunuyor. Fonksiyonel olarak zaten emekli (adapter reject, mint header-chain'e bağlı). Yani "legacy kod tamamen kaldırıldı" denemez — tip bağımlılık nedeniyle duruyor.

## 3. Mint-gate test bağımlılığı

Legacy self-declared PoW'yü BAŞARILI bir mint'e bağlayan hiçbir test YOK. Mevcut `pow_finality_legacy_adapter_always_rejects_after_d3` testi TAM TERSİNİ (her zaman reject) doğruluyor. Dolayısıyla kaldırılacak "legacy'ye bağlı mint-gate testi" bulunmuyor.

## 4. Karar Noktası (Ayaz'a soruluyor)

Task C: *"Kaldırılamıyorsa (bağımlılık varsa), gerekçesini yaz ve Ayaz'a sor."* Bağımlılık (bincode stabilitesi + historical-compat) GERÇEK. İki yol:

- **(A) Tipi koru (önerilen):** `FinalityProof::PoW` varyantı bincode stabilitesi için tutulur; fonksiyonel olarak zaten emekli. C yalnızca bu doğrulama raporu ile kapanır, kod değişikliği YOK. Breaking değişiklik ve veri-format riski yok.
- **(B) Tamamen kaldır (breaking):** Varyant + `PoWFinalityAdapter` + historical branch kaldırılır; PoW domain'leri `PoWHeaderChainFinalityAdapter`'a yönlendirilir; serileştirme migration gerekir. Breaking change, tarihsel commitment deserialize riski, çoklu CI round-trip.

Detaylar ve öneri için STATUS_ONLINE + bu rapor. Karar Ayaz'da.
