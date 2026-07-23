# Kalan İşler — Budlum

**Güncelleme:** 2026-07-22 (ARENA2) · main

---

## AI execution layer

✅ **Hardened skeleton:** host MLP eval, domain commitments, guest v2, structural+STARK envelope checks, `require_execution_proof` finalization gate, `prove_mlp_inference`, `try_finalize_with_proofs`.

✅ **VerifyInference AIR binding (ARENA2 2026-07-23):** 5 column (373-377), selector↔opcode 0x1F binding, rd=0 fail-closed, expansion row commitment consistency, VM next_pc fix, soundness test.

✅ **Dense matmul host bit-exact:** `eval_fixed_point_mlp` i32 MAC host'ta; guest sadece Poseidon commitment bind.

✅ **Full matmul-in-guest AIR (ARENA2 2026-07-23):** `build_matmul_guest_program()` — MLP forward pass BudZKVM instruction'larında (Load/Mul/Add/Sub/Lt/Jnz/Poseidon/Halt). Register allocation, ReLU conditional, MAX_GUEST_OPS guard. 7 test.

✅ **Production gas metering (ARENA2 2026-07-23):** `estimate_structural_gas`, `estimate_full_gas`, `validate_gas_budget`. Executor `AiAttachExecutionProof` path'inde proof size vs execution class limit kontrolü.

📋 **LLM / float:** mainnet v1 scope dışı — `docs/AI_ONCHAIN_EXECUTION_RESEARCH.md`.

## CI güvenlik

✅ **CodeQL workflow** (ARENA2 2026-07-23): `.github/workflows/codeql.yml` — Rust CodeQL, security-extended, weekly schedule.

✅ **cargo-vet config** (ARENA2 2026-07-23): `supply-chain/` — kademeli benimseme, FFI/unsafe crate hedef.

✅ **actionlint + zizmor**: zaten mevcut (Repo Lint job).

✅ **Dockerfile devnet default** (ARENA2 2026-07-23).

## Intent → zincir (private transfer)

✅ L1NoteRegistry + PrivateTransferSubmit path.

## Z-B / HSM / TEE / Ceremony

Z-B ✅ · HSM ⏳ donanım · TEE ⏳ SDK · Ceremony ⏳ Ayaz.

---
*CI tek hakem.*
