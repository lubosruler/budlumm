# Status Online — Aktif iletişim kanalı (AI birliği)

**Amaç:** İki (veya daha fazla) AI'ın anlık olarak ne yaptığını, ne yapacağını,
karar taleplerini ve engelleri burada paylaşması. Statik denetim kayıtları
(`docs/STATUS.md`) ve şema (`docs/AI_BIRLIGI.md`) buraya girmez — bu
dosya **çalışma günlüğü**.

**Format:** serbest, ama timestamp'li ve AI-handle imzalı. Eski entry
"resolved" notuyla kalır (audit trail).

**Yazan:** her iki AI (handle + tarih).
**Okuyan:** her iki AI + kullanıcı (PR'a bakıyor olabilir).

---

## Entry şablonu

```markdown
### [YYYY-MM-DD HH:MM UTC+3] <handle> — <özet>

**Durum:** [devam ediyor | tamamlandı | engel | karar bekliyor]
**Kapsam:** [Tur X §Y | kod | docs | test | infra]
**Aksiyon:** ne yaptım / ne yapıyorum
**Kanıt:** [commit SHA | dosya | komut çıktısı]
**Sonraki adım:** ne yapacağım
**Engel (varsa):** ne durduruyor / ne karar bekliyor
```

---

## 2026-07-14 — kuruluş günü

### [2026-07-14 19:00 UTC+3] arena-agent — dosya şeması kuruldu

**Durum:** tamamlandı
**Kapsam:** docs (AI birliği şeması)
**Aksiyon:** `docs/DEVIR_RAPORU.md` → `docs/AI_BIRLIGI.md` rename + 4. bölümde
DEVIR_RAPORU'nun tamamı korunmuş + §1 AI üyeleri tablosu (2 → 4+ dinamik)
+ §2 iletişim kanalları ayrımı + §3 aktif iş akışı + §6 kesin kurallar.
`docs/STATUS.md` "tek-ajan devir" → "statik denetim" perspektifine
güncellendi (HEAD `39e30c7` referansı, 8 commit listesi).
`docs/STATUS_ONLINE.md` (bu dosya) yeni oluşturuldu — aktif iletişim
kanalı.

**Kanıt:** `git status` → 3 dosya değişti: rename + 2 modify (AI_BIRLIGI rename
zaten git mv ile staged).

**Sonraki adım:** kullanıcı doğrulaması → commit + push.

**Engel:** kullanıcı "the-plan yerine ekledim" / "oraya budlum zip ekledim"
diye belirsizlik yarattı — `/tmp/the-plan-check/` lokal kopya mevcut, içinde
TUR14_PLAN + TUR14_5_PLAN + claude-fable-5.md var (PR'a eklenmedi, sadece
referans olarak kullanıldı). zip değil, açılmış dizin. Beklemede.

### [2026-07-14 19:05 UTC+3] arena-agent — bekleme

**Durum:** karar bekliyor
**Kapsam:** repo state
**Aksiyon:** eklenen dosyalar hakkında kullanıcıdan net bilgi bekleniyor
(`/tmp/the-plan-check/` budlum zip midir? PR'a katılacak mı?). Şu an
working tree: `AI_BIRLIGI.md` (rename) + `STATUS.md` (modify) + `STATUS_ONLINE.md`
(new) staged değil, kullanıcı onayı sonrası tek commit'te push edilecek.

**Kanıt:** `git status --short` → 3 dosya değişiklik (2 modify + 1 rename).

**Sonraki adım:** kullanıcı cevabı → tek commit'te `ai-birligi-sema` mesajıyla push.

**Engel:** kullanıcının "zip ekledim" / "the-plan ekledim" mesajları netleşmedi;
eklediği dosyalar PR'a katılmalı mı yoksa sadece referans mı?

---

## Çözülmüş entry'ler (aşağıya taşı)

_(henüz yok — kuruluş günü)_
