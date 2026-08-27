#!/system/bin/sh
MODDIR="$(cd "$(dirname "$0")" && pwd)"

print_log() {
    if [ "$FROM_WEBUI" != "1" ]; then
        echo "$*"
    fi
}

print_log "========================================="
print_log " Schedule Wallpaper - Manual Refresh"
print_log "========================================="
print_log ""

# Pindah ke direktori config
cd "$MODDIR/system/etc/"

print_log "[-] Mengeksekusi modul generator jadwal..."
"$MODDIR/system/bin/schedule-wallpaper" 2>&1 | while read -r line; do print_log "    $line"; done

print_log "[-] Menyalin gambar ke /sdcard/Pictures..."
mkdir -p /sdcard/Pictures
cp "jadwal_kuliah.png" "/sdcard/Pictures/jadwal_kuliah.png"

print_log "[-] Menerapkan wallpaper baru..."
mkdir -p /data/local/tmp
cp "$MODDIR/system/bin/SetWallpaper.jar" /data/local/tmp/SetWallpaper.jar
cp "jadwal_kuliah.png" /data/local/tmp/jadwal_kuliah.png
chmod 777 /data/local/tmp/SetWallpaper.jar /data/local/tmp/jadwal_kuliah.png

# Eksekusi via UID 1000 (System), fallback ke UID 2000 (Shell) atau root
APPLIED=0
if su 1000 -c "app_process -cp /data/local/tmp/SetWallpaper.jar /system/bin SetWallpaper /data/local/tmp/jadwal_kuliah.png" 2>&1 | while read -r line; do print_log "    $line"; done; then
    APPLIED=1
fi

if [ "$APPLIED" != "1" ]; then
    print_log "[-] Mencoba fallback UID 2000..."
    su 2000 -c "app_process -cp /data/local/tmp/SetWallpaper.jar /system/bin SetWallpaper /data/local/tmp/jadwal_kuliah.png" 2>&1 | while read -r line; do print_log "    $line"; done
fi

rm -f /data/local/tmp/SetWallpaper.jar /data/local/tmp/jadwal_kuliah.png

print_log ""
print_log "Selesai! Wallpaper berhasil diperbarui."

if [ "$FROM_WEBUI" != "1" ]; then
    print_log ""
    print_log "Membuka UI Module..."
    su -c "am start -a android.intent.action.VIEW -d 'file://$MODDIR/webroot/index.html' -t 'text/html'" > /dev/null 2>&1
fi
