# ARENA2 — AccessGrant Kullanıcı Karar Kaydı

**Tarih:** 2026-07-18 (UTC+3)
**Durum:** Kullanıcı kararı kaydı; uygulama kodu değildir.

## Karar 1 — Her ödeme belirli bir veri kapsamı içindir

Bir ödeme, yalnız satın alma anında açıkça tanımlanmış veri parçasına ve kullanım hakkına karşılık gelir.

- Yeni veri, daha geniş veri veya yeniden erişim için yeni izin ve yeni ödeme gerekir.
- Veri parçası sonradan değişmez. İçerik değişirse yeni bir veri kaydı oluşturulur; önceki izin otomatik olarak yeni içeriğe geçmez.
- Veriyi daha önce alan kişinin bir kopyayı saklaması teknik olarak engellenemez. Daha sonra değişen veya satın alınmamış veri alanları için eski izin geçerli değildir.

**Önerilen ürün dili:** “Ödeme geçmişte satın aldığınız kapsamı kapsar; gelecekte eklenen veya değiştirilmiş veriyi kapsamaz.”

## Karar 2 — Tek kullanımlık izin ayrı kullanım kaydıyla izlenir

Tek kullanımlık izin kullanıldığında zincirde ayrı bir “kullanıldı” kaydı oluşur.

- Aynı izin ikinci kez kullanılmak istendiğinde reddedilir.
- İzin belgesi ile kullanım kaydı ayrı tutulur; böylece kimin, ne zaman ve hangi izinle eriştiği daha açık denetlenir.
- Bu kayıt, daha önce kopyalanmış veriyi geri almaz; yalnız sonraki yetkili erişimleri yönetir.

## Uygulama öncesi açık tasarım noktaları

1. Bir veri parçasının sınırı: tüm manifest, belirli shard grubu veya başka sabit bir kapsam mı?
2. Her ödeme için izin kime verilir: belirli kullanıcı/verifier adresine mi, yoksa yalnız belirli bir iş isteğine mi?
3. Otomatik satışta owner’ın önceden verdiği satış izninin süre, alıcı, fiyat ve kapsam sınırları.
4. Kullanım kaydının hangi işlemle oluşacağı ve işlem ücretinin nasıl karşılanacağı.

Bu kararlar, AccessGrant RFC’sindeki `RoleId` tabanlı grant yaklaşımının yerine **adres-temelli ve kapsamı açık erişim** yönünü destekler.
