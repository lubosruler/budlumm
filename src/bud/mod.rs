//! B.U.D. (Broad Universal Database) — Phase 10 marketplace/izin katmanı.
//!
//! P0 (AccessGrant v2, APPROVED): temel tipler `marketplace` modülünde yaşar.
//! Kök `src/storage/` sağlayıcı ekonomisini, bu modül tüketici-erişim
//! (izin/provenance/marketplace) katmanını taşır — ikisi ayrıdır (RFC §5).

pub mod marketplace;
