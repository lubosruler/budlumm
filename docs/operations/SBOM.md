# SBOM (Software Bill of Materials) (Tur 15 §1.7)

> **Otomatik üretilir:** `./scripts/generate-sbom.sh` çalıştırıldığında
> `sbom.cdx.json` üretilir ve bu dosya yenilenir. CI'da `sbom` job'ı
> bu scripti çalıştırır.

**Araç:** [cargo-cyclonedx](https://github.com/CycloneDX/cyclonedx-rust-cargo)
**Format:** CycloneDX 1.5 (JSON)
**Kapsam:** Tüm `Cargo.toml` + transitive bağımlılıklar (budzero dahil).

## Kullanım

```bash
./scripts/generate-sbom.sh
```

**Çıktı:** `sbom.cdx.json` (repo root) + `docs/operations/SBOM.md` özeti.

## Neden SBOM?

- **ch12 §3.7 mainnet blocker:** Harici audit + mainnet launch için
  zorunlu teslim kalemi.
- **Supply chain güvenliği:** Bilinen CVE'leri, lisans uyumluluğunu,
  transitive bağımlılık risklerini görünür kılar.
- **Compliance:** Bazı regülasyonlar (AB NIS2, ABD EO 14028) SBOM
  zorunluluğu getiriyor.

## CycloneDX formatı

CycloneDX, OWASP tarafından yönetilen açık SBOM standardı. JSON + XML
formatlarını destekler. Bu repo JSON formatını kullanır.

Örnek alanlar:
- `components[]` — her bağımlılık (purl, version, license)
- `dependencies[]` — bağımlılık grafiği
- `metadata` — araç, zaman damgası, repo bilgisi

## Kabul kriteri

- `sbom.cdx.json` oluşturulabiliyor.
- JSON parse oluyor (doğrulama: `python3 -c "import json; json.load(open('sbom.cdx.json'))"`).
- Bileşen sayısı 0'dan büyük.
- CI'da `sbom` job'ı her push'ta bu scripti çalıştırır.

## Tur 15 kapsamı

Bu doküman Tur 15 §1.7 kapsamında oluşturuldu. **Tur 15 kapanışında:**
- ✅ `scripts/generate-sbom.sh` mevcut
- ✅ `sbom.cdx.json` oluşturulabiliyor (CI ilk çalıştırmada)
- ✅ `docs/operations/SBOM.md` mevcut
- ⏳ İlk SBOM üretilecek (CI ilk çalıştırmada)

## İlgili

- `scripts/generate-sbom.sh` — script
- `docs/operations/DEPENDENCY_AUDIT.md` — bağımlılık audit
- `the-plan/TUR15_PLAN.md` §1.7 — plan referansı
