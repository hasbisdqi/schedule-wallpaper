#!/bin/bash

# Pastikan berada di dalam folder modul
cd "$(dirname "$0")"

echo "=== Memulai Proses Build SetWallpaper.jar ==="

# Cek apakah Java terinstall
if ! command -v javac &> /dev/null; then
    echo "❌ Java (javac) tidak ditemukan di laptop Anda!"
    echo "Silakan install Java terlebih dahulu. Jika menggunakan Homebrew, jalankan:"
    echo "brew install openjdk"
    exit 1
fi

# Cari Android SDK platforms (gunakan versi terbaru yang ada, misal android-33/34)
SDK_PLATFORM=$(ls -d ~/Library/Android/sdk/platforms/android-* 2>/dev/null | tail -n 1)
if [ -z "$SDK_PLATFORM" ]; then
    echo "❌ Android SDK (android.jar) tidak ditemukan!"
    echo "Pastikan Anda sudah menginstall Android Studio beserta SDK-nya."
    exit 1
fi

# Cari d8 (DEX compiler dari Android SDK build-tools)
D8_TOOL=$(ls -d ~/Library/Android/sdk/build-tools/*/d8 2>/dev/null | tail -n 1)
if [ -z "$D8_TOOL" ]; then
    echo "❌ Tool 'd8' tidak ditemukan di Android SDK!"
    exit 1
fi

echo "[1/3] Mengkompilasi SetWallpaper.java (target Java 8)..."
javac --release 8 -cp "$SDK_PLATFORM/android.jar" SetWallpaper.java

if [ $? -ne 0 ]; then
    echo "❌ Gagal mengkompilasi Java!"
    exit 1
fi

echo "[2/3] Mengubah ke format DEX (SetWallpaper.jar)..."
"$D8_TOOL" SetWallpaper.class --output system/bin/SetWallpaper.jar

if [ $? -ne 0 ]; then
    echo "❌ Gagal mengubah ke DEX!"
    exit 1
fi

# Bersihkan file .class
rm SetWallpaper.class

echo "[3/3] Mengirim file ke HP (pastikan HP tersambung ADB)..."
adb push system/bin/SetWallpaper.jar /data/local/tmp/
adb shell su -c "cp /data/local/tmp/SetWallpaper.jar /data/adb/modules/schedule-wallpaper/system/bin/SetWallpaper.jar"
adb shell su -c "chmod 644 /data/adb/modules/schedule-wallpaper/system/bin/SetWallpaper.jar"

echo "✅ Berhasil! SetWallpaper.jar sudah diperbarui di laptop dan di HP."
echo "Sekarang coba tekan kembali tombol Action di aplikasi Magisk Anda."
