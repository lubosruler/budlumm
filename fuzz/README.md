# Fuzzing (Tur 15 §1.7)

> **Durum:** Setup tamamlandı (Tur 15 §1.7 kapsamı). Fuzzing run'u
> Tur 16+ kapsamında manuel olarak çalıştırılır.

## Fuzz target'leri

| Target | Amaç | Durum |
|--------|------|-------|
| `fuzz_blockchain_serialize` | Blockchain serialization roundtrip | ✅ Mevcut (minimal) |

## Çalıştırma

**Önkoşul:** Rust nightly toolchain (sadece fuzzing için).

```bash
# 1. Nightly yükle (sadece bir kez)
rustup install nightly

# 2. cargo-fuzz yükle
cargo +nightly install cargo-fuzz

# 3. Fuzz target çalıştır (30 saniye, sonra Ctrl+C)
cargo +nightly fuzz run fuzz_blockchain_serialize

# 4. Bulgu raporları
ls fuzz/artifacts/fuzz_blockchain_serialize/
```

## CI entegrasyonu

**Tur 15'te CI'da fuzzing çalıştırılmaz** (aşağıdaki nedenlerle):
- Fuzzing run uzun sürer (saatler/günler), CI dakikalarla sınırlı.
- `cargo-fuzz` nightly gerektirir, CI stable kullanıyor.
- Bulgu varsa crash raporu artifact olarak kaydedilir; bu akış Tur 16+.

CI'da sadece **build kontrolü** (Tur 15 §1.7 kabul kriteri):
- `cargo check --manifest-path fuzz/Cargo.toml` (nightly ile)
- Veya fuzz dizini `.gitignore` ile muaf tutulur (Tur 16+ kararı).

## Kabul kriteri

- ✅ `fuzz/Cargo.toml` mevcut
- ✅ `fuzz/fuzz_targets/` en az 1 target içeriyor
- ✅ `cargo check --manifest-path fuzz/Cargo.toml` temiz
- ⏳ Fuzzing run'u Tur 16+'da

## İlgili

- `fuzz/Cargo.toml` — fuzz workspace
- `fuzz/fuzz_targets/fuzz_blockchain_serialize.rs` — ilk target
- `the-plan/TUR15_PLAN.md` §1.7 — plan referansı
