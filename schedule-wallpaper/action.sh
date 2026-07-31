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
cd "$MODDIR/system/etc/schedule-wallpaper/"

print_log "[-] Mengeksekusi modul generator jadwal..."
"$MODDIR/system/bin/schedule-wallpaper" 2>&1 | while read -r line; do print_log "    $line"; done

print_log "[-] Menyalin gambar ke /sdcard/Pictures..."
cp "jadwal_kuliah.png" "/sdcard/Pictures/jadwal_kuliah.png"

print_log "[-] Menerapkan wallpaper baru..."
su 2000 -c "app_process -cp \"$MODDIR/system/bin/SetWallpaper.jar\" /system/bin SetWallpaper \"/sdcard/Pictures/jadwal_kuliah.png\"" 2>&1 | while read -r line; do print_log "    $line"; done

print_log ""
print_log "Selesai! Wallpaper berhasil diperbarui."

if [ "$FROM_WEBUI" != "1" ]; then
    print_log ""
    print_log "Membuka UI Module..."
    am start -a android.intent.action.VIEW -t text/html -d "file://$MODDIR/webroot/index.html" >/dev/null 2>&1
fi
