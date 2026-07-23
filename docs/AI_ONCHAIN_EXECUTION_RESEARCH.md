# On-Chain AI Execution Layer — Hardened v2

**Durum:** Sertleştirilmiş iskelet main'de (`src/ai/execution/`).  
**Paradigma:** `docs/03_paradigma_analizi.md` §5 Agentic Economy.

## Attestation vs Execution

| | Attestation | Execution |
|---|---|---|
| Soru | "k verifier aynı çıktıyı onayladı mı?" | "çıktı bu model+input'tan mı?" |
| Kod | `AiRegistry` k-of-n | host MLP + guest STARK + L1 attach |

## Hardening (v2)

1. **Host bit-exact MLP** — `eval_fixed_point_mlp` (i32 MAC, ReLU hidden).
2. **Domain commitments** — `BDLM_AI_INPUT_V1` / `BDLM_AI_OUTPUT_V1`.
3. **Guest v2** — Poseidon(weights_limb, input_limb) + Log + Halt; `program_hash` binds weights.
4. **`prove_mlp_inference`** — eval + guest + STARK (`prove_bytecode`) + postcard envelope.
5. **Structural verify** — commitments, model, non-empty proof, program_hash ≠ 0, model bind.
6. **STARK verify path** — postcard `ProofEnvelope` deserialize + size/degree/backend allow-list on L1 attach.
7. **Policy** — `require_execution_proof=true` → agreement only counts verifiers with attached proof; `try_finalize_with_proofs` after attach.

## Fail-closed rules

- Empty / oversized proof_bytes → reject.
- Non-postcard envelope → reject on L1 attach.
- Model `execution_program_hash` set → must match proof.
- Finalization cannot complete on execution-class models without proofs.

## Non-goals (still)

- Full dense matmul inside STARK guest (gas); host eval is authoritative for numbers.
- LLM / float.
- Auto mainnet gate open.
