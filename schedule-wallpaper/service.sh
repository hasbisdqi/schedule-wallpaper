#!/system/bin/sh

# Tunggu sampai HP selesai booting sepenuhnya
until [ "$(getprop sys.boot_completed)" = "1" ]; do
    sleep 5
done

# Pindah ke direktori tempat config dan assets berada
cd /system/etc/schedule-wallpaper

# Infinite loop (berjalan terus di background)
while true; do
    # 1. Jalankan program Rust-mu
    /system/bin/schedule-wallpaper

    # 2. Pindahkan hasil gambar ke penyimpanan internal agar mudah diakses
    # (Pastikan di kode Rust-mu, output gambarnya disimpan di folder saat ini)
    cp jadwal_kuliah.png /sdcard/Pictures/jadwal_kuliah.png
    
    # 3. SET WALLPAPER
    # Catatan: Perintah ini tergantung pada versi Android dan ROM (MIUI/OneUI/AOSP).
    # Opsi A (Android 13+ AOSP):
    # cmd wallpaper set /sdcard/Pictures/jadwal_kuliah.png
    
    # Tunggu selama 15 menit (900 detik) sebelum mengulang lagi
    sleep 900
done