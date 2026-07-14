# External Audit Checklist (Tur 15 §1.5)

**Tarih:** 2026-07-14
**Durum:** Tur 15 kapsamında oluşturuldu (checklist hazır; audit yapılmadı).
**Sorumlu:** Audit firması seçimi kullanıcı kararıdır.

> **Önemli:** Bu doküman bir **teslim paketi**dir, "audit tamamlandı"
> iddiası taşımaz. Harici audit **yapılmamıştır**; sadece audit
> başlangıcında gereken tüm materyaller listelenmiştir.

## 1. Kod tabanı

### 1.1 Repository
- [ ] `git clone https://github.com/lubosruler/budlum.git`
- [ ] `git checkout <release tag veya commit>`
- [ ] README.md, CLAUDE.md, docs/ klasörü

### 1.2 Build kanıtı
- [ ] `cargo build --release --locked` çıktısı
- [ ] `cargo build --release --manifest-path budzero/Cargo.toml --locked`
- [ ] Reproducible build kanıtı (deterministic hash, sha256sum)

### 1.3 Test coverage
- [ ] `cargo test --lib --verbose` çıktısı
- [ ] `cargo test --lib --manifest-path budzero/Cargo.toml --workspace --verbose`
- [ ] Test coverage raporu (`cargo tarpaulin` veya `grcov`)
- [ ] Code coverage % (kritik modüller için ≥ %80)

### 1.4 Static analysis
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] `cargo fmt --all -- --check` (diff yok)
- [ ] Clippy lints listesi + suppress edilenler (allow list)

## 2. Dependency & supply chain

### 2.1 SBOM
- [ ] `sbom.cdx.json` (CycloneDX 1.5 format, `scripts/generate-sbom.sh`)
- [ ] Tüm transitive bağımlılıklar dahil
- [ ] License uyumluluğu (GPL/AGPL yok, MIT/Apache-2.0 tercih)

### 2.2 Dependency audit
- [ ] `docs/operations/DEPENDENCY_AUDIT.md` (cargo-audit raporu)
- [ ] Bilinen CVE yok (veya "accepted risk" listesi)
- [ ] Unmaintained bağımlılık listesi (false positive gözden geçirilmiş)

### 2.3 Dependency graph
- [ ] `cargo tree` çıktısı
- [ ] `cargo metadata --format-version=1` (JSON)

## 3. Fuzzing & dynamic analysis

### 3.1 Fuzz setup
- [ ] `fuzz/Cargo.toml` + en az 1 fuzz target (`fuzz/fuzz_targets/`)
- [ ] `cargo +nightly fuzz check` (build temiz)
- [ ] Fuzzing run raporu (`fuzz/artifacts/`, crash reproducer)

### 3.2 Property-based testing
- [ ] `proptest` veya `quickcheck` entegrasyonu (varsa)
- [ ] Shrinking sonuçları (varsa)

## 4. Operasyonel dokümanlar

### 4.1 Runbook'lar (`docs/operations/`)
- [x] `PRODUCTION_RUNBOOK.md` — Production runbook (Tur 13.5)
- [x] `ARCHIVE_AND_BACKUP.md` — Archive + backup (Tur 13.5)
- [ ] `HSM_BLS_PQ_POLICY.md` — BLS/PQ HSM policy (Tur 15 §1.1)
- [ ] `FINALITY_LIVE_PATH.md` — Finality live-path tarama (Tur 15 §1.3)
- [ ] `MIGRATION_V2.md` — ConsensusStateV2 migration (Tur 15 §1.4)
- [ ] `MAINNET_LAUNCH_CHECKLIST.md` — Mainnet launch (Tur 16.9)

### 4.2 Incident response
- [x] Production runbook içinde incident response akışı
- [ ] On-call rotation prosedürü (varsa)
- [ ] Severity classification (P0/P1/P2)
- [ ] Post-mortem template

### 4.3 Threat model
- [ ] `docs/SECURITY_MODEL.md` — trust assumptions, attack vectors
- [ ] Her attack vector için mitigation
- [ ] Out-of-scope saldırılar (sosyal mühendislik, vb.)

## 5. Kriptografi

### 5.1 Algoritma envanteri
- [ ] Ed25519 — mevcut (`src/crypto/pkcs11.rs`)
- [ ] BLS12-381 — Tur 15 §1.1
- [ ] Dilithium5 (PQ) — Tur 15 §1.1
- [ ] Keccak-256 (Ethereum uyumluluğu) — mevcut
- [ ] Poseidon4 (B.U.D. Faz 2) — Tur 15 §1.2
- [ ] Diğer (libp2p noise/ yamux, vs.) — mevcut

### 5.2 HSM entegrasyonu
- [ ] Ed25519 PKCS#11 — mevcut, audit edilmiş
- [ ] BLS/PQ PKCS#11 — Tur 15 §1.1 (mock backend, gerçek HSM mainnet öncesi)
- [ ] Disk key policy — README:88 + `HSM_BLS_PQ_POLICY.md`

## 6. Konsensüs

### 6.1 Multi-consensus
- [ ] PoW finality adapter — mevcut, bounded
- [ ] PoS finality adapter — mevcut
- [ ] PoA finality adapter — mevcut, isolated
- [ ] BFT finality adapter — mevcut
- [ ] Storage finality adapter — Tur 15 §1.2 (B.U.D. Faz 1)
- [ ] Custom domains — mevcut

### 6.2 Cross-domain
- [ ] BridgeState (lock/mint/burn/unlock) — mevcut
- [ ] CrossDomainMessage — mevcut, forgery-gated
- [ ] PoW mint gate — Tur 13.5

### 6.3 Finality
- [ ] Finality live-path test raporu — Tur 15 §1.3
- [ ] Adversarial test coverage — `src/tests/finality_adversarial.rs`

## 7. Network & RPC

### 7.1 RPC
- [ ] Per-IP quota — Tur 13.5
- [ ] Body/connection limitleri — Tur 13.5
- [ ] Operator-only admin methods — Tur 13.5
- [ ] Latency histogram — Tur 13.5

### 7.2 P2P
- [ ] libp2p integration — mevcut (`kad`, `gossipsub`, `noise`, `yamux`)
- [ ] Discovery + DNS seed — mevcut
- [ ] Banned peers — mevcut

## 8. Storage

### 8.1 L1 storage
- [x] Snapshot V2 (archive policy) — Tur 13.5
- [x] Atomic backup + restore drill — Tur 13.5
- [x] Retention policy — Tur 13.5

### 8.2 B.U.D. (Tur 15 §1.2)
- [ ] ContentManifest (CID) — Tur 15
- [ ] StorageRegistry (permissionless) — Tur 15
- [ ] 3-aktör E2E testi — Tur 15
- [ ] Ekonomik parametreler — Tur 15 (StorageEconomicsParams)
- [ ] Faz 3+ (VerifyMerkle bağımlı) — Tur 17+

## 9. Privacy & AI

### 9.1 Privacy layer
- ❌ Araştırma — kod yok
- [ ] Privacy roadmap (varsa)

### 9.2 AI execution layer
- ❌ Araştırma — kod yok
- [ ] AI execution roadmap (varsa)

## 10. Kabul kriteri (audit başlangıcı)

Bu checklist'teki her madde için repo'da referans mevcut. **Audit
başlamadan önce:**

1. Tüm maddeler ✅ (varsa) veya "not done" olarak işaretli.
2. SBOM + dependency audit raporu CI'da üretilmiş.
3. Fuzz setup mevcut.
4. Tüm runbook'lar (`docs/operations/`) mevcut.
5. Test coverage raporu mevcut.
6. Clippy + fmt CI'da yeşil.

**Audit "tamamlandı" iddiası audit raporunun kapsamına göredir
(limited assurance vs full assurance).** Bu checklist audit raporu
DEĞİLDİR.

## İlgili

- `the-plan/TUR15_PLAN.md` §1.5 — plan referansı
- `docs/operations/DEPENDENCY_AUDIT.md` — dependency audit
- `docs/operations/SBOM.md` — SBOM
- `docs/DEVIR_RAPORU.md` — devir raporu
- `README.md` — "Research Roadmap Status" tablosu
