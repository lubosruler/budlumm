# On-Chain AI Execution Layer — Araştırma İskeleti

**Durum:** Araştırma / tasarım iskeleti (KALAN_ISLER). Production kod yok.
**Tarih:** 2026-07-22 · **Hazırlayan:** ARENA2
**Ayrım:** Mevcut `src/ai/` = **attestation + soft-incentive** (off-chain inference,
on-chain k-of-n agreement). Bu doküman = modelin **zincirde çalıştırılıp
STARK ile kanıtlanması** (zkML / BudZKVM guest).

---

## 1. Neden ayrı hat

| Katman | Ne yapar | Durum |
|---|---|---|
| Lubot / AiRegistry | Model kayıt, request/result attestation, threshold agreement, escrow | ✅ kod var |
| VerifyInference opcode (0x1F) | ZKVM proof envelope doğrulama iskeleti | 🔧 stub (her zaman 0) |
| **On-chain AI execution** | Model weights + input → output, STARK-provable | ❌ araştırma |

Attestation, "operatör X şu çıktıyı iddia etti, k verifier onayladı" der.
Execution, "bu çıktı gerçekten bu model+input'tan geldi" kriptografik kanıtıdır.
İkisi tamamlayıcıdır; attestation mainnet v1 için yeterli olabilir, execution
değil.

## 2. Kısıtlar (2026 pratik)

1. **Model sınıfı:** Yalnızca küçük, tamamen belirleyici (bit-exact) modeller —
   quantize MLP / küçük transformer subset. Büyük LLM zkML hâlâ pratik değil.
2. **Field:** BudZKVM Goldilocks + Poseidon; float yok → fixed-point / integer
   nets.
3. **Gas / trace:** Tek inference trace_len ve gas_limit bütçesine sığmalı;
   aksi halde recursive proof / continuation gerekir (v2).
4. **İzolasyon:** NFT / B.U.D. / Pollen state'ine dokunmaz (gizlilik katmanı
   Bölüm 7 ile aynı izolasyon disiplini).

## 3. Önerilen mimari (iskelet)

```
AiModelSpec (mevcut)
  └─ program_hash: BudZKVM guest bytecode hash (weights+graph tied)
AiExecutionRequest
  ├─ model_id
  ├─ input_commitment = Poseidon(input_blob)
  └─ gas_limit
AiExecutionProof (mevcut attach_execution_proof yolu)
  ├─ ProofEnvelope (Plonky3)
  ├─ public: program_hash, input_commitment, output_commitment, gas
  └─ VerifyInference opcode (0x1F) mainnet-gated
```

**Guest program:** budl/ veya raw ISA ile yazılmış fixed-point net evaluator.
**Host:** Lubot operatörü input'u (Pollen grant ile) okur, guest'i çalıştırır,
proof üretir, `attach_execution_proof` ile zincire koyar.
**Verifier:** `VerifyInference` + registry agreement.

## 4. Açık sorular (kod yazmadan önce)

1. Hangi model sınıfları v1 whitelist'te? (parametre üst sınırı, op seti)
2. Weights on-chain mi (commitment only) yoksa B.U.D. content-addressed mı?
3. VerifyInference AIR: mevcut stub → gerçek ProofEnvelope verify gadget
   maliyeti kabul edilebilir mi?
4. Attestation-only mainnet v1, execution v1.1 mi? (öneri: **execution v1.1+**)

## 5. Bilinçli non-goals

- Genel amaçlı LLM'i zincirde çalıştırmak.
- FHE ile gizli inference (ayrı TEE/FHE hattı; cüzdan TEE opt-in ile kısmen örtüşür).
- Mevcut attestation API'sini kırmak.

## 6. Sonraki somut adımlar (öncelik sırası)

1. Bounded model-class whitelist RFC (parametre + op subset).
2. Mini fixed-point MLP budl guest + prove/verify round-trip (bud-proof).
3. VerifyInference AIR: ProofEnvelope public-input binding (0x1F gate açmadan).
4. AiRegistry ↔ execution proof zorunluluğu policy flag (opt-in per model).

---

*KALAN_ISLER "AI execution layer" maddesi bu iskeletle kapatılmaz — kod yok.
Araştırma hattı açıldı; production iddiası yok.*
