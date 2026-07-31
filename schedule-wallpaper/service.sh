#!/system/bin/sh

# Ini akan secara akurat mencari di mana file script ini berada, 
# lalu mengambil path foldernya secara absolut (misal: /data/adb/modules/jadwal_kuliah)
MODDIR="$(cd "$(dirname "$0")" && pwd)"

until [ "$(getprop sys.boot_completed)" = "1" ]; do
    sleep 5
done

# Pindah ke folder tempat 'config' dan 'assets' berada
cd "$MODDIR/system/etc/schedule-wallpaper/"

# Buat log file
echo "=== JADWAL KULIAH LOG ===" > /sdcard/jadwal_log.txt
echo "[$(date)] MODDIR terdeteksi di: $MODDIR" >> /sdcard/jadwal_log.txt

while true; do
    echo "[$(date)] Mengeksekusi program Rust..." >> /sdcard/jadwal_log.txt
    
    # Eksekusi program
    "$MODDIR/system/bin/schedule-wallpaper" >> /sdcard/jadwal_log.txt 2>&1
    
    # Copy hasil
    cp "jadwal_kuliah.png" "/sdcard/Pictures/jadwal_kuliah.png" >> /sdcard/jadwal_log.txt 2>&1
    
    # Eksekusi Java class SetWallpaper menggunakan app_process
    # (Asumsi: SetWallpaper.java sudah dicompile menjadi SetWallpaper.jar dan disimpan di $MODDIR/system/bin/)
    app_process -cp "$MODDIR/system/bin/SetWallpaper.jar" /system/bin SetWallpaper "/sdcard/Pictures/jadwal_kuliah.png" >> /sdcard/jadwal_log.txt 2>&1
    
    echo "[$(date)] Selesai." >> /sdcard/jadwal_log.txt
    echo "--------------------------------" >> /sdcard/jadwal_log.txt
    
    sleep 900
done