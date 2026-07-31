#!/system/bin/sh

# Ini akan secara akurat mencari di mana file script ini berada, 
# lalu mengambil path foldernya secara absolut (misal: /data/adb/modules/jadwal_kuliah)
MODDIR="$(cd "$(dirname "$0")" && pwd)"

until [ "$(getprop sys.boot_completed)" = "1" ]; do
    sleep 5
done

# Pindah ke folder tempat 'config' dan 'assets' berada
cd "$MODDIR/system/etc/"

# Buat log file
echo "=== JADWAL KULIAH LOG ===" > /sdcard/jadwal_log.txt
echo "[$(date)] MODDIR terdeteksi di: $MODDIR" >> /sdcard/jadwal_log.txt

while true; do
    echo "[$(date)] Mengeksekusi program Rust..." >> /sdcard/jadwal_log.txt
    
    # Eksekusi program
    "$MODDIR/system/bin/schedule-wallpaper" >> /sdcard/jadwal_log.txt 2>&1
    
    # Copy hasil
    cp "jadwal_kuliah.png" "/sdcard/Pictures/jadwal_kuliah.png" >> /sdcard/jadwal_log.txt 2>&1
    
    # Eksekusi Java class SetWallpaper menggunakan app_process dengan UID 2000 (Shell)
    # File jar dicopy ke /data/local/tmp agar bisa diakses oleh UID 2000 (menghindari Aborted)
    cp "$MODDIR/system/bin/SetWallpaper.jar" /data/local/tmp/SetWallpaper.jar
    cp "jadwal_kuliah.png" /data/local/tmp/jadwal_kuliah.png
    chmod 777 /data/local/tmp/SetWallpaper.jar /data/local/tmp/jadwal_kuliah.png
    su 1000 -c "app_process -cp /data/local/tmp/SetWallpaper.jar /system/bin SetWallpaper /data/local/tmp/jadwal_kuliah.png" >> /sdcard/jadwal_log.txt 2>&1
    rm /data/local/tmp/SetWallpaper.jar /data/local/tmp/jadwal_kuliah.png
    
    echo "[$(date)] Selesai." >> /sdcard/jadwal_log.txt
    echo "--------------------------------" >> /sdcard/jadwal_log.txt
    
    sleep 900
done