# Budlum Mimari Atlası

> **Durum:** Kod haritası ve hedef mimariyi birlikte gösterir. Diyagramlardaki
> `feature-gated`, `planlı` ve `mainnet blocker` etiketleri tamamlanmış üretim
> özelliği iddiası değildir.

## 1. Genel sistem mimarisi

```mermaid
flowchart TB
  Client[Wallet / dApp / Operator] --> RPC[JSON-RPC + admission]
  RPC --> MP[Mempool]
  MP --> Chain[ChainActor / Blockchain]
  Chain --> Exec[Transaction Executor]
  Chain --> Consensus[PoW / PoS / PoA / BFT finality]
  Exec --> State[(AccountState)]
  Consensus --> State
  State --> Store[(Durable storage + snapshots)]
  Chain --> Net[P2P / Gossip]
  Exec --> ZK[BudZero / BudZKVM]
  State --> XD[Cross-domain / bridge state]
  State --> Apps[BNS · B.U.D. · Pollen · Hub · SocialFi · AI]
```

## 2. Consensus-domain izolasyonu

```mermaid
flowchart LR
  POW[Permissionless PoW] --> FA[DomainFinalityAdapter]
  POS[Permissionless PoS] --> FA
  BFT[Permissionless BFT] --> FA
  ZK[ZK proof domain] --> FA
  POA[Isolated PoA / KYC membership] --> PFA[PoA finality adapter]
  FA --> Settlement[Global settlement commitment]
  PFA --> Settlement
  Reg[Permissionless stake registry] -. never shared .-> POA
  PoAReg[Separate PoA membership registry] -. isolated .-> Reg
```

## 3. Transaction admission and V4 signing

```mermaid
sequenceDiagram
  participant C as Client
  participant R as RPC / P2P decoder
  participant M as Mempool
  participant E as Executor
  C->>C: Canonical V4 payload + signature
  C->>R: Transaction
  R->>R: signature_version == V4?
  R->>R: canonical hash + signature verify
  R->>M: admitted transaction
  M->>E: selected block transaction
  E->>E: nonce / balance / type rules
  E-->>C: state transition or typed rejection
```

## 4. Cross-domain bridge lifecycle

```mermaid
flowchart LR
  Lock[Lock on source domain] --> Event[Committed lock event]
  Event --> Proof[Relayer proof]
  Proof --> Verify[Header / receipt / MPT verification]
  Verify --> Mint[Mint on target]
  Mint --> Burn[Burn on target]
  Burn --> BurnProof[Verified burn event]
  BurnProof --> Unlock[Unlock on original source]
  Verify --> Replay[Replay / correlation / state-machine gates]
  Replay --> Mint
  Replay --> Unlock
```

## 5. EVM receipt verification path

```mermaid
flowchart TB
  Headers[Target header + confirmation chain] --> HC[Parent hash / height checks]
  Receipt[Receipt envelope bytes] --> RLP[Strict RLP decoder]
  Proof[MPT proof nodes + receipt key] --> MPT[Merkle-Patricia verifier]
  HC --> Root[receiptsRoot]
  Root --> MPT
  RLP --> Log[Status + emitter + topic0 + payload checks]
  MPT --> ReceiptValue[Committed receipt bytes]
  ReceiptValue --> RLP
  Log --> Deposit[Verified deposit facts]
```

## 6. Snapshot trust and schema migration

```mermaid
flowchart LR
  Live[AccountState] --> S4[Schema-4 canonical digest]
  S4 --> Sign[Manifest signer / signature]
  Sign --> Disk[(Snapshot storage)]
  Disk --> Load[Loader]
  Load --> Integrity[Digest + field manifest verify]
  Integrity --> Auth[External trust policy verify]
  Auth --> Restore[Restore state]
  Integrity --> Quarantine[Quarantine / fail-loud]
  Auth --> Quarantine
  Legacy[Schema-2/3 legacy import] --> Versioned[Version-specific digest]
  Versioned --> Integrity
```

## 7. Critical durability boundary

```mermaid
flowchart LR
  Input[Verified bridge / QC / finality input] --> Stage[Stage next state]
  Stage --> Batch[Atomic durable batch]
  Batch --> Flush[Apply + flush]
  Flush --> Publish[Publish in-memory state]
  Flush -->|failure| Stop[Fail-stop / operator recovery]
  Publish --> Next[Relay / finality continuation]
```

## 8. BudZero execution and proof boundary

```mermaid
flowchart LR
  Contract[Contract bytecode] --> VM[bud-vm execution]
  VM --> Trace[Execution trace]
  Trace --> Proof[bud-proof / STARK proof]
  Proof --> Verify[Budlum verifier]
  Verify --> Exec[Executor state transition]
  VM --> Host[Host calls: AI request / chain integration]
```

## 9. AI inference lifecycle

```mermaid
flowchart LR
  Model[Model registration] --> Request[Inference request + fee escrow]
  Request --> Verifiers[Permissionless AI verifiers]
  Verifiers --> Results[Signed result commitments]
  Results --> Threshold[Agreement / equivocation checks]
  Threshold --> Outcome[Finalized outcome + callback]
  Threshold --> Reclaim[Expired no-consensus fee reclaim]
```

## 10. B.U.D. storage lifecycle

```mermaid
flowchart LR
  Content[Content manifest + shards] --> Deal[Permissionless storage deal]
  Deal --> Operator[Storage operator]
  Operator --> Challenge[Interim retrieval challenge]
  Challenge --> Outcome[Challenge outcome]
  Gate[VerifyMerkle 64-depth gate] -. required for real Proof-of-Storage .-> Production[Production proof-of-storage]
```

## 11. Mainnet launch gates

```mermaid
flowchart TD
  V29[V29 signing V4] --> P2[P2 snapshot integrity]
  P2 --> V19[Critical durability]
  V19 --> Audit[External audit / bounty]
  Audit --> Fuzz[Long-running fuzz campaign]
  Fuzz --> HSM[Vendor-native HSM decision/test]
  HSM --> Ceremony[Genesis ceremony + bootnodes]
  Ceremony --> Freeze[Genesis hash freeze]
  Freeze --> Launch[Mainnet launch approval]
  ZKGate[VerifyMerkle soundness or explicit feature gate] --> Launch
```

## 12. CI and security gates

```mermaid
flowchart LR
  Push[Commit / PR] --> Fmt[Format + lint]
  Fmt --> Core[Core / BudZero tests]
  Core --> Invariants[BNS · B.U.D. · PoA invariant gates]
  Invariants --> Coverage[Coverage ratchet]
  Coverage --> Supply[deny · SBOM · secret scan · geiger · udeps]
  Supply --> Smoke[Docker / multinode smoke]
  Smoke --> CI[CI verdict]
  Fuzz[Fuzz quick + nightly campaigns] -. ongoing evidence .-> CI
```
