# Dependency Audit Raporu

**Oluşturulma:** 2026-07-14T23:20:36Z
**Araç:** cargo-audit (https://github.com/rustsec/rustsec)
**Repo:** lubosruler/budlum @ `3d3f6ba`

## Özet

- ⚠️ cargo-audit exit code: 1 (genelde unmaintained warning).

## Ham çıktı

```
    Fetching advisory database from `https://github.com/RustSec/advisory-db.git`
      Loaded 1160 security advisories (from /home/user/.cargo/advisory-db)
    Updating crates.io index
warning: directory /home/user/.cargo/advisory-db is locked, waiting for up to 300 seconds for it to become available
    Scanning Cargo.lock for vulnerabilities (501 crate dependencies)
Crate:     crossbeam-epoch
Version:   0.9.18
Title:     Invalid pointer dereference in `fmt::Pointer` impl for `Atomic` and `Shared` when the underlying pointer is invalid
Date:      2026-07-06
ID:        RUSTSEC-2026-0204
URL:       https://rustsec.org/advisories/RUSTSEC-2026-0204
Solution:  Upgrade to >=0.9.20

Crate:     hickory-proto
Version:   0.24.4
Title:     CPU exhaustion during message encoding due to O(n²) name compression
Date:      2026-05-01
ID:        RUSTSEC-2026-0119
URL:       https://github.com/hickory-dns/hickory-dns/security/advisories/GHSA-q2qq-hmj6-3wpp
Solution:  Upgrade to >=0.26.1

Crate:     protobuf
Version:   2.28.0
Title:     Crash due to uncontrolled recursion in protobuf crate
Date:      2024-12-12
ID:        RUSTSEC-2024-0437
URL:       https://rustsec.org/advisories/RUSTSEC-2024-0437
Solution:  Upgrade to >=3.7.2

Crate:     quinn-proto
Version:   0.11.14
Title:      Remote memory exhaustion in quinn-proto from unbounded out-of-order stream reassembly
Date:      2026-06-22
ID:        RUSTSEC-2026-0185
URL:       https://rustsec.org/advisories/RUSTSEC-2026-0185
Severity:  7.5 (high)
Solution:  Upgrade to >=0.11.15

Crate:     ring
Version:   0.16.20
Title:     Some AES functions may panic when overflow checking is enabled.
Date:      2025-03-06
ID:        RUSTSEC-2025-0009
URL:       https://rustsec.org/advisories/RUSTSEC-2025-0009
Solution:  Upgrade to >=0.17.12

Crate:     rustls-webpki
Version:   0.101.7
Title:     Name constraints for URI names were incorrectly accepted
Date:      2026-04-14
```

## Kabul kriteri

CI'da `dependency-audit` job'ı bu scripti çalıştırır. **Bilinen
güvenlik açığı (CVE) tespit edilirse job fail eder.** Unmaintained
warning'leri warning olarak raporlanır (fail etmez).

Bu rapor Tur 15 §1.7 kapsamında otomatik üretilir.
