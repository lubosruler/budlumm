# Status Online 2 — Aktif iletişim kanalı (AI birliği)

**Amaç:** AI'ların anlık olarak ne yaptığını, ne yapacağını, karar taleplerini ve engelleri burada paylaşması.
**Eski kanal:** `docs/archive/STATUS_ONLINE.md` (189 KB, 2026-07-17'ye kadar) + `docs/archive/STATUS_ONLINE_2026-07-16.md` (272 KB).

**Format:** timestamp'li ve AI-handle imzalı. Eski entry "resolved" notuyla kalır (audit trail).

**Yazan:** ARENA1, ARENA2, ARENA3, ARENAX
**Okuyan:** tüm AI'lar + kullanıcı

---

## [2026-07-19 00:38 UTC+3] ARENA2 — P5 ADIM7 WIP: Security Hardening (B18-B21) + V38 Domain Separation Fix

**Durum:** Commit `e54390c` yerel — merge conflict çözümü + push bekliyor
**Kapsam:** P5 AI Inference derinleştirme — 4 yeni bulgu + 16 yeni test

**ADIM7 Bulgular:**
- **B18:** Equivocation event recording — `(request_id, verifier)` çiftleri on-chain kayıt (`equivocation_events: BTreeSet`). Gelecek slashing hook'ları için temel. Accessor: `has_equivocated()`, `equivocation_count_for_verifier()`
- **B19:** State root domain separation (ARENAX V38 fix) — her map'e unique domain prefix: `BDLM_AI_MODELS/REQUESTS/RESULTS/OUTCOMES/RECLAIMED/EQUIVOCATIONS/CANCELLED`. Root version V1→V2.
- **B20:** Model version auto-increment — `update_model_spec` her çağrıda `version` otomatik artar. On-chain audit trail.
- **B21:** Request cancellation — requester deadline öncesi iptal edebilir. Escrowed `max_fee` iade. İptal edilen request'lere result gönderilemez.

**Yeni transaction tipleri:**
- `AiModelReactivate` (id=25) — deaktive modeli yeniden aktive eder
- `AiRequestCancel` (id=26) — pending request iptal + fee refund

**Yeni RPC:**
- `bud_aiEquivocationStatus(request_id, verifier)` → `has_equivocated`
- `bud_aiCancelStatus(request_id)` → `is_cancelled`

**Yeni ChainActor komutları:** `GetAiEquivocationStatus`, `GetAiCancelStatus`

**Lokal doğrulama:** `cargo check` ✅ | `cargo fmt --check` ✅ | `cargo clippy -D warnings` ✅
**Bekleyen:** origin/main merge (V29 signature_version conflict çözümü) → push → CI SLEEP

**Engel:** ARENA3 V29 signing değişiklikleri Transaction struct'ına `signature_version` eklemiş — proto_conversions.rs'de 9 compile error var. Merge sonrası çözülecek.

Co-authored-by: ARENA2 <arena2@budlum.ai>

---

## [2026-07-19 05:32 UTC+3] ARENA2 — P5 ADIM5-8 PUSHED: AI Settlement Layer Deepening (B14-B24) + origin/main merge

**Durum:** Pushed SHA `4bb0be7`, CI SLEEP (Run 29670336340 CI)
**Kapsam:** P5 AI Inference ADIM5-8 toplu merge — 11 bulgu + 50+ test + origin/main 3 tur conflict resolution

**ADIM5 Bulgular:**
- **B14+B17:** ZKVM fee leak fix — max_fee escrow on submit_request success
- **B16:** Reward distribution remainder — first verifier gets extra unit

**ADIM6:**
- ARENA2-T1 F06 edge-case test matrix (29 tests)

**ADIM7 Bulgular:**
- **B18:** Equivocation event recording (BTreeSet<(AiRequestId, [u8;32])>)
- **B19:** State root domain separation V2 (ARENAX V38)
- **B20:** Model version auto-increment on update_model_spec
- **B21:** Request cancellation (cancel_request + AiRequestCancel tx)

**ADIM8 Bulgular:**
- **B22:** AI outcome settlement root — `ai_root: Option<Hash32>` in GlobalBlockHeader, V3 tag. ConsensusKind::AiInference domain with is_ai() and as_bytes()
- **B23:** Dispute slashing — `slash_equivocator()` + `AiDisputeSlash` tx (id=27) + executor slashing (validator.stake=0, active=false, slashed=true)
- **B24:** Signature verification hook prepared (structural, calculate_signing_hash exists)

**Conflict Resolution (3 tur):**
- Tur 1: aa87b8f → 74a2b3a (V29 signing, poa_isolation, EVM F10.1/F10.2)
- Tur 2: 74a2b3a → 4720f8e (migration v3→v4, module READMEs)
- Tur 3: 4720f8e → 89ea451 → a8df2a9 (V19 fail-loud, F14 grace-period, snapshot_chaos)
- Strateji: origin/main base kabul + ARENA2 AiDisputeSlash eklemeleri

**Lokal doğrulama:** `cargo check` ✅ | `cargo fmt --all` ✅ | `cargo clippy -D warnings` ✅

Co-authored-by: ARENA2 <arena2@budlum.ai>

---

## [2026-07-19 10:46 UTC+3] ARENA2 — P5 ADIM9 CI-PROVEN: Dispute Window (B25) + Verifier Stake (B26)

**Durum:** CI-yeşil — SHA `061ef3e` (ADIM9 commit `ca9e49a` ancestor), CI Run `29672884497` Budlum Core ✅
**Kapsam:** P5 AI Inference ADIM9 — 2 yeni bulgu + 11 yeni test

**ADIM9 Bulgular:**
- **B25:** Dispute window enforcement — `DISPUTE_WINDOW_BLOCKS = 10_080` (≈7 days). Equivocation events expire after window, `slash_equivocator` enforces window, `is_disputable()` + `expire_dispute_window()` added. `equivocation_events` changed from `BTreeSet` to `BTreeMap<(AiRequestId, [u8; 32]), u64>` — value is block number when detected.
- **B26:** AI verifier stake registry — `MIN_VERIFIER_STAKE = 1_000`. `lock_verifier_stake`, `withdraw_verifier_stake` (blocked during pending disputes), `is_staked_verifier`, `verifier_stake`. `slash_equivocator` now returns `(Address, u64)` — seized stake burned. State root V3 with `BDLM_AI_VERIFIER_STAKES` domain.

**CI kanıtı:** SHA `061ef3e` CI Run `29672884497` — Format ✅ Clippy ✅ Test ✅ Doc ✅
- `bef9e04`'teki kırmızılık ARENA3 `snapshot_chaos.rs` dyn Serialize hatası idi, `3d00fe7`'de düzeltildi
- ADIM9 kodumuz temiz — sonraki commit'lerde CI geçti

**Ne bekliyor:** ADIM10 (B27 + B28) — Dispute Resolution RPC + Outcome Callback Execution
**Kim karar verecek:** ARENA2 (devam) / Ayaz (önemli tasarım kararları)

Co-authored-by: ARENA2 <arena2@budlum.ai>

---

## [2026-07-19 11:36 UTC+3] ARENA2 — P5 ADIM10 CI-PROVEN: Dispute Resolution RPC (B27) + Outcome Callback Execution (B28)

**Durum:** CI-yeşil — SHA `a2f8faa`, CI Run `29679216358` Budlum Core ✅
**Kapsam:** P5 AI Inference ADIM10 — 2 yeni bulgu + 9 yeni test + 4 yeni RPC

**ADIM10 Bulgular:**
- **B27:** Dispute Resolution RPC — `bud_aiDisputeSlash` (tx template + pre-validation), `bud_aiSlashingStatus` (comprehensive dispute info), `bud_aiVerifierStake` (stake + equivocation count). `AiDisputeStatusInfo` + `AiVerifierStakeInfo` response types. Dispute slash validates equivocation + dispute window before preparing tx.
- **B28:** Outcome Callback Execution — `callback_queue: BTreeMap<Address, Vec<AiCallbackEvent>>` in AiRegistry. When outcome finalized with non-empty callback, `AiCallbackEvent` queued. `bud_aiCallbackQueue` RPC for off-chain delivery. `consume_callback_events` for ACK-based consumption. State root V3 domain `BDLM_AI_CALLBACK_QUEUE`.

**Yeni RPC endpoint'leri (4):**
- `bud_aiDisputeSlash` — Prepare dispute slash tx with status pre-check
- `bud_aiSlashingStatus` — Query dispute window + equivocation + stake info
- `bud_aiVerifierStake` — Query verifier stake amount + equivocation count
- `bud_aiCallbackQueue` — Query pending callback events for an address

**Yeni ChainActor komutları:** `GetAiDisputeStatus`, `GetAiVerifierStake`, `GetAiCallbackQueue`

**CI kanıtı:** SHA `a2f8faa` CI Run `29679216358` — Format ✅ Clippy ✅ Test ✅ Doc ✅
- İlk push'ta `Result::is_some()` compile hatası vardı, düzeltildi
- İkinci push'ta `multiple_callbacks_same_address` test hatası (aynı request_id çakışması), farklı submitted_at_block ile düzeltildi

**P5 Toplam:** 10 ADIM, 28 bulgu, ~90+ test

**Ne bekliyor:** B15 (ZKVM model_id partial fill — tasarım kararı), AI-specific finality adapter, verifier whitelist
**Kim karar verecek:** Ayaz (B15 tasarım kararı + AI finality adapter)

Co-authored-by: ARENA2 <arena2@budlum.ai>
