# 0.2.1

- Menampilkan seluruh jadwal dari Senin hingga Jumat secara penuh (tidak collapsible)
- Jadwal selain hari ini dirender dengan warna abu-abu redup (dim gray) agar tetap terbaca rapi
- Highlight terang tetap aktif pada hari ini dan mata kuliah yang sedang berlangsung

---

- Full rendering of weekly schedule (non-collapsible)
- Non-active days rendered in subtle dim gray
- Bright highlight for active day and current class

# 0.2.0

- Penambahan Web UI Dashboard Interaktif untuk mengatur jadwal (CRUD) secara langsung
- Dukungan deteksi KernelSU API untuk sinkronisasi simpan jadwal secara instan dan update wallpaper otomatis
- Desain UI baru (Dark mode, Glassmorphism, Responsive)

---

- Added Interactive Web UI Dashboard for direct schedule management (CRUD)
- Added KernelSU API detection for instant schedule sync and auto wallpaper update
- New UI design (Dark mode, Glassmorphism, Responsive)

# 0.1.2

- Memperbaiki kegagalan memuat gambar akibat Scoped Storage di Android 13/14+ dengan mengeksekusi Java menggunakan UID 1000 (System)
- Menambahkan skrip `build_java.sh` untuk otomatisasi kompilasi Java

---

- Fixed image loading failure due to Scoped Storage on Android 13/14+ by executing Java using UID 1000 (System)
- Added `build_java.sh` script for automated Java compilation

# 0.1.1

- Memperbaiki masalah crash saat memasang wallpaper (Invalid package) di Android 13/14+
- Menambahkan WebUI (Dashboard HTML) untuk KernelSU / APatch
- Memfungsikan tombol Action (di Magisk) sebagai tombol "Manual Refresh" untuk memperbarui wallpaper seketika

---

- Fix crash issue when applying wallpaper (Invalid package) on Android 13/14+
- Add WebUI (HTML Dashboard) support for KernelSU / APatch
- Implement Magisk Action button as a "Manual Refresh" trigger to update wallpaper instantly

# 0.1.0

- Rilis perdana modul Schedule Wallpaper

---

- Initial release of Schedule Wallpaper module
