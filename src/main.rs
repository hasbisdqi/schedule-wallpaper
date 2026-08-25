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
    let color_dim = Rgba([120, 120, 120, 255]); // Abu-abu untuk jadwal hari lain
    let color_highlight = Rgba([255, 204, 0, 255]); // Kuning terang untuk kelas yang sedang berlangsung

    let mut components = Vec::new();

    // Dapatkan waktu saat ini menggunakan chrono
    let now = Local::now();
    let current_day = now.format("%A").to_string(); // Contoh: "Monday"
    let current_minutes = now.hour() * 60 + now.minute();

    // Judul "PROGRAM" diturunkan ke Y: 850
    components.push(Components::Text(150, 850, 75, "PROGRAM", color_white, None));

    let mut current_y = 1000;

    for daily in schedule {
        let day_upper = daily.day.to_uppercase();
        let day_str: &'static str = Box::leak(day_upper.into_boxed_str());

        let is_today = daily.day.eq_ignore_ascii_case(&current_day);
        let header_color = if is_today { color_white } else { color_dim };

        // Nama Hari
        components.push(Components::Text(
            150,
            current_y,
            42,
            day_str,
            header_color,
            None,
        ));
        current_y += 55;

        if daily.classes.is_empty() {
            let free_color = if is_today { color_white } else { color_dim };
            components.push(Components::Text(
                220,
                current_y,
                26,
                "N O   C L A S S E S",
                free_color,
                None,
            ));
            current_y += 55;
        } else {
            for class in daily.classes {
                let subject_str: &'static str = Box::leak(class.subject.into_boxed_str());
                let room_str: &'static str = Box::leak(class.room.into_boxed_str());
                let time_str: &'static str = Box::leak(class.time.into_boxed_str());

                let text_color = if is_today {
                    if is_current_class(time_str, current_minutes) {
                        color_highlight
                    } else {
                        color_white
                    }
                } else {
                    color_dim
                };

                // Kolom 1: Mata Kuliah
                components.push(Components::Text(
                    220,
                    current_y,
                    32,
                    subject_str,
                    text_color,
                    None,
                ));

                // Kolom 2: Ruangan
                components.push(Components::Text(
                    560, current_y, 32, room_str, text_color, None,
                ));

                // Kolom 3: Waktu
                components.push(Components::Text(
                    840, current_y, 32, time_str, text_color, None,
                ));

                current_y += 42;
            }
            current_y += 25; // Jarak antar hari
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
