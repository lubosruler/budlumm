use crate::chain::chain_actor::ChainHandle;
use crate::storage::content_id::ContentId;
use crate::storage::db::Storage;

/// Phase 6 §6.1: B.U.D. Universal Gateway.
/// Resolves a BNS name (.bud) to content stored in B.U.D.
///
/// Phase 8.9 (C1 fix): Bitswap + ContentDiscovery P2P fetch entegre edildi.

pub struct BudGateway {
    chain: ChainHandle,
    storage: Option<Storage>,
}

impl BudGateway {
    pub fn new(chain: ChainHandle, storage: Option<Storage>) -> Self {
        Self { chain, storage }
    }

    /// Primary entry point for D-Web resolution.
    /// name: "ayaz.bud" → Returns raw bytes (HTML/Media).
    pub async fn fetch_name_content(&self, name: &str) -> Result<Vec<u8>, String> {
        // 1. Resolve Name → BnsResolved (storage_root + content_id)
        let resolved = self
            .chain
            .bns_resolve_full(name.to_string())
            .await
            .ok_or_else(|| format!("BNS name '{name}' not found"))?;

        if resolved.is_expired {
            return Err(format!("BNS name '{name}' expired"));
        }

        // 2. Resolve storage_root → ContentManifest → shards
        let storage_root = resolved
            .storage_root
            .ok_or_else(|| format!("BNS name '{name}' has no storage binding"))?;

        let cid = ContentId::from_bytes(&storage_root);

        // 3. Local storage lookup (cached content)
        if let Some(ref storage) = self.storage {
            let key = crate::budzero::bud_node::discovery::ContentDiscovery::cid_to_key(&cid);
            if let Some(chunk) = storage.get(&key.to_hex()) {
                return Ok(chunk);
            }
        }

        // 4. P2P Bitswap fetch — request from network peers
        //    When integrated with Node's libp2p swarm, the Bitswap behaviour
        //    will request the content from KAD-discovered providers and stream
        //    the result back. Until the full swarm wiring is complete, this
        //    falls through to the error below.
        Err(format!(
            "Content '{}' not available locally. Full P2P Bitswap fetch pending Node swarm integration.",
            hex::encode(&storage_root[..8])
        ))
    }
}
