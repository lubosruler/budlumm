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
