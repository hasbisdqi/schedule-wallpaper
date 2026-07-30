use chrono::{Local, Timelike};
use omage::colors::*;
use omage::{Components, Config, Image, Rgba};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
struct ClassInfo {
    subject: String,
    room: String,
    time: String,
}

#[derive(Deserialize, Debug)]
struct DaySchedule {
    day: String,
    classes: Vec<ClassInfo>,
}

const HEIGHT: u32 = 2400;
const WIDTH: u32 = 1080;

// Fungsi bantuan untuk mengubah "10:00" menjadi total menit dari tengah malam (600)
fn parse_time_to_minutes(time_str: &str) -> u32 {
    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() == 2 {
        let h: u32 = parts[0].parse().unwrap_or(0);
        let m: u32 = parts[1].parse().unwrap_or(0);
        return h * 60 + m;
    }
    0
}

// Fungsi untuk mengecek apakah waktu sekarang berada di dalam rentang jam kuliah
fn is_current_class(time_range: &str, current_minutes: u32) -> bool {
    // Memisahkan rentang waktu berdasarkan strip (mendukung hyphen biasa '-' atau en-dash '–')
    let parts: Vec<&str> = time_range.split(|c| c == '-' || c == '–').collect();
    if parts.len() == 2 {
        let start = parse_time_to_minutes(parts[0].trim());
        let end = parse_time_to_minutes(parts[1].trim());
        return current_minutes >= start && current_minutes <= end;
    }
    false
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_data = fs::read_to_string("config/schedule.json").expect("Gagal membaca jadwal.json");
    let schedule: Vec<DaySchedule> = serde_json::from_str(&json_data)?;

    let config = Config::new(
        WIDTH,
        HEIGHT,
        Rgba([0, 0, 0, 255]),
        Some(BLACK),
        "jadwal_kuliah.png",
        Some("./assets/GeistMono-Medium.ttf"),
    );

    let mut image = Image::new();

    // Palet Warna
    let color_white = Rgba([255, 255, 255, 255]);
    let color_dim = Rgba([120, 120, 120, 255]); // Abu-abu untuk badge hari lain
    let color_highlight = Rgba([255, 204, 0, 255]); // Kuning terang untuk kelas yang sedang berlangsung

    let mut components = Vec::new();

    // Dapatkan waktu saat ini menggunakan chrono
    let now = Local::now();
    let current_day = now.format("%A").to_string(); // Contoh: "Monday"
    let current_minutes = now.hour() * 60 + now.minute();

    // Judul "PROGRAM"
    components.push(Components::Text(150, 800, 75, "PROGRAM", color_white, None));

    let mut current_y = 950;

    for daily in schedule {
        let day_upper = daily.day.to_uppercase();
        let day_str: &'static str = Box::leak(day_upper.into_boxed_str());

        // Cek apakah hari pada loop ini adalah hari ini
        if daily.day.eq_ignore_ascii_case(&current_day) {
            // == RENDER JADWAL HARI INI SECARA PENUH ==

            // Nama Hari (kiri)
            components.push(Components::Text(
                150,
                current_y,
                45,
                day_str,
                color_white,
                None,
            ));
            current_y += 60;

            if daily.classes.is_empty() {
                components.push(Components::Text(
                    350,
                    current_y,
                    25,
                    "N O   C L A S S E S   T O D A Y",
                    color_white,
                    None,
                ));
                current_y += 90;
            } else {
                for class in daily.classes {
                    let subject_str: &'static str = Box::leak(class.subject.into_boxed_str());
                    let room_str: &'static str = Box::leak(class.room.into_boxed_str());
                    let time_str: &'static str = Box::leak(class.time.into_boxed_str());

                    // Gunakan `time_str` karena ia sudah berwujud &str yang valid,
                    // sedangkan `class.time` sudah tidak bisa dipakai lagi.
                    let text_color = if is_current_class(time_str, current_minutes) {
                        color_highlight
                    } else {
                        color_white
                    };

                    // Kolom 1: Mata Kuliah
                    components.push(Components::Text(
                        220,
                        current_y,
                        35,
                        subject_str,
                        text_color,
                        None,
                    ));

                    // Kolom 2: Ruangan
                    components.push(Components::Text(
                        550, current_y, 35, room_str, text_color, None,
                    ));

                    // Kolom 3: Waktu
                    components.push(Components::Text(
                        840, current_y, 35, time_str, text_color, None,
                    ));

                    current_y += 45;
                }
                current_y += 60; // Jarak antar hari
            }
        } else {
            // == RENDER HARI LAIN SEBAGAI BADGE (REKAPAN) ==
            let class_count = daily.classes.len();

            let badge_text = if class_count == 0 {
                format!("{} - FREE", day_str)
            } else {
                format!("{} - {} CLASSES", day_str, class_count)
            };

            let badge_str: &'static str = Box::leak(badge_text.into_boxed_str());

            // Kita tampilkan sedikit lebih kecil dan warnanya redup (abu-abu)
            components.push(Components::Text(
                150, current_y, 30, badge_str, color_dim, None,
            ));

            current_y += 50;
        }
    }

    let component_refs: Vec<_> = components.iter().collect();

    image
        .config(config)
        .init()?
        .add_components(component_refs)
        .draw()?;

    println!("Gambar jadwal_kuliah.png berhasil di-render!");
    Ok(())
}
