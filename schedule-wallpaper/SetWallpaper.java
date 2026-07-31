import android.app.WallpaperManager;
import android.content.Context;
import android.graphics.Bitmap;
import android.graphics.BitmapFactory;
import android.os.Looper;
import java.lang.reflect.Method;

public class SetWallpaper {
    public static void main(String[] args) {
        if (args.length < 1) {
            System.err.println("Error: Masukkan path gambar!");
            System.err.println("Cara pakai: app_process -cp /system/bin/SetWallpaper.jar /system/bin SetWallpaper /sdcard/gambar.jpg");
            return;
        }

        // 1. Menyiapkan jalur proses utama agar API Android tidak crash
        Looper.prepareMainLooper();

        try {
            // 2. Mendapatkan System Context secara paksa menggunakan Reflection 
            // (Karena API ini disembunyikan oleh sistem Android)
            Class<?> activityThreadClass = Class.forName("android.app.ActivityThread");
            Method systemMainMethod = activityThreadClass.getDeclaredMethod("systemMain");
            Object activityThread = systemMainMethod.invoke(null);

            Method getSystemContextMethod = activityThreadClass.getDeclaredMethod("getSystemContext");
            Context systemContext = (Context) getSystemContextMethod.invoke(activityThread);

            // 3. Panggil WallpaperManager dengan systemContext bawaan
            // systemContext menggunakan package "android" yang cocok dengan UID 1000 (System)
            WallpaperManager wallpaperManager = WallpaperManager.getInstance(systemContext);
            
            // 4. Membaca gambar dari argumen skrip
            String imagePath = args[0];
            Bitmap bitmap = BitmapFactory.decodeFile(imagePath);

            if (bitmap != null) {
                // 5. Menerapkan gambar ke background layar
                wallpaperManager.setBitmap(bitmap);
                System.out.println("Sukses! Wallpaper berhasil diubah di latar belakang.");
            } else {
                System.err.println("Gagal! File gambar tidak ditemukan atau rusak.");
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
        
        // 6. Matikan proses setelah selesai agar tidak nyangkut di memori
        System.exit(0);
    }
}
