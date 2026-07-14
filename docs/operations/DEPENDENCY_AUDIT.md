# Dependency Audit (Tur 15 §1.7)

> **Otomatik üretilir:** `./scripts/audit-deps.sh` çalıştırıldığında bu
> dosya yenilenir. CI'da `dependency-audit` job'ı bu scripti çalıştırır.

**Araç:** [cargo-audit](https://github.com/rustsec/rustsec)
**Format:** RustSec Advisory Database
**Kapsam:** Tüm `Cargo.toml` + transitive bağımlılıklar (budzero dahil).

## Kullanım

```bash
./scripts/audit-deps.sh
```

**Çıktı:** `docs/operations/DEPENDENCY_AUDIT.md` (bu dosya) + stdout.

## Kabul kriteri

- Bilinen güvenlik açığı (CVE) tespit edilirse **CI fail eder**.
- `unmaintained` warning'leri **CI fail etmez** (ayrıca gözden geçirilir).
- Audit çalıştırma zamanı ve commit hash rapora yazılır.

## CI entegrasyonu

`.github/workflows/ci.yml` → `dependency-audit` job'ı her push'ta bu
scripti çalıştırır. Başarısız olursa PR merge edilemez.

## Tur 15 kapsamı

Bu doküman Tur 15 §1.7 "Fuzzing + dependency audit + SBOM" kapsamında
oluşturuldu. **Tur 15 kapanışında:**
- ✅ `scripts/audit-deps.sh` mevcut
- ✅ `docs/operations/DEPENDENCY_AUDIT.md` mevcut
- ✅ CI job'ı entegre
- ⏳ İlk audit raporu üretilecek (CI ilk çalıştırmada)

## İlgili

- `scripts/audit-deps.sh` — script
- `docs/operations/SBOM.md` — SBOM dokümanı
- `the-plan/TUR15_PLAN.md` §1.7 — plan referansı
