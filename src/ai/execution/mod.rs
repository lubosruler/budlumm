//! On-chain AI **execution** primitives (paradigm shift #5 — Agentic Economy).
//!
//! Hardened surface:
//! - bounded model-class whitelist
//! - host bit-exact fixed-point MLP eval + domain commitments
//! - guest bytecode (weights+input limb Poseidon bind) + program_hash
//! - structural verify + optional STARK verify (postcard ProofEnvelope)
//! - prove_mlp_inference packages AiExecutionProof for L1 attach

mod guest;
mod model_class;
mod verify;

pub use guest::{
    build_fixed_point_mlp_guest, eval_fixed_point_mlp, input_commitment, output_commitment,
    program_hash_from_words, prove_mlp_inference, weights_digest, words_to_bytecode,
    FixedPointMlpSpec, MLP_GUEST_VERSION,
};
pub use model_class::{
    AiExecutionModelClass, ModelClassLimits, DEFAULT_EXECUTION_CLASS, MAX_MLP_LAYERS,
    MAX_MLP_PARAMS, MAX_MLP_WIDTH,
};
pub use verify::{
    verify_execution_proof_full, verify_execution_proof_stark, verify_execution_proof_structural,
    verify_execution_proof_structural_with_model, ExecutionVerifyReport,
};
