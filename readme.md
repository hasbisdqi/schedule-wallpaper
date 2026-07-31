# Schedule Wallpaper

Schedule Wallpaper adalah modul Magisk / KernelSU / APatch yang dirancang untuk secara dinamis memperbarui *wallpaper lockscreen* atau layar utama Anda dengan jadwal kuliah/sekolah sehari-hari.

Modul ini dibuat menggunakan **Rust** (dengan library `omage`) untuk men-generate gambar secara on-the-fly dan Java (`app_process`) untuk mengatur wallpaper secara langsung di *background* (berjalan dengan izin *Root*).

## ✨ Fitur
- 📅 **Jadwal Dinamis**: Menampilkan jadwal secara penuh untuk hari ini, dan rekap untuk hari-hari lainnya.
- 🟡 **Highlight Otomatis**: Mata kuliah yang sedang berlangsung saat ini akan otomatis di-highlight dengan warna cerah.
- 🔄 **Pembaruan Otomatis**: Wallpaper otomatis diperbarui secara berkala (default setiap 15 menit) di latar belakang.
- 🚀 **Manual Refresh**: Mendukung tombol *Action* di Magisk atau WebUI (KernelSU / APatch) untuk memperbarui wallpaper saat itu juga tanpa perlu menunggu.
- 🌐 **OTA Update**: Mendukung pembaruan otomatis (Over-The-Air) langsung dari aplikasi Magisk/KernelSU.

## 📦 Pemasangan
1. Unduh file `Schedule-Wallpaper-vX.X.X.zip` terbaru dari halaman [Releases](../../releases).
2. Buka aplikasi Magisk / KernelSU / APatch.
3. Masuk ke tab **Modules**, pilih **Install from storage**.
4. Pilih file `.zip` yang sudah diunduh dan tunggu proses *flashing* selesai.
5. *Reboot* perangkat Anda.

## ⚙️ Konfigurasi
Anda perlu menyesuaikan jadwal bawaan dengan jadwal asli Anda sendiri. Konfigurasi jadwal disimpan dalam format JSON.

1. Buka File Manager yang mendukung Root (seperti MT Manager atau Root Explorer).
2. Pergi ke direktori modul: `/data/adb/modules/schedule-wallpaper/system/etc/schedule-wallpaper/config/`
3. Edit file `schedule.json` sesuai jadwal Anda.
   *(Gunakan format penulisan waktu HH:MM-HH:MM, contoh: "08:00-10:00")*
4. Simpan file tersebut.
5. Buka Magisk/KernelSU lalu tekan tombol **Action** pada modul ini untuk menerapkan jadwal baru Anda secara instan.
