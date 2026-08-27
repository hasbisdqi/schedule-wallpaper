use chrono::{Local, Timelike};
use omage::colors::*;
use omage::{Components, Config, Image, Rgba};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug, Clone, Default)]
struct StickerConfig {
    #[serde(default)]
    id: String,
    #[serde(default = "default_sticker_type")]
    r#type: String,
    #[serde(default)]
    content: String,
    #[serde(default)]
    x: u32,
    #[serde(default)]
    y: u32,
    #[serde(default = "default_sticker_size")]
    size: u32,
}

fn default_sticker_type() -> String {
    "emoji".to_string()
}
fn default_sticker_size() -> u32 {
    64
}

#[derive(Deserialize, Debug, Clone)]
struct WallpaperSettings {
    #[serde(default = "default_bg_type")]
    background_type: String,
    #[serde(default = "default_bg_color")]
    background_color: String,
    #[serde(default)]
    background_image: String,
    #[serde(default = "default_color_active")]
    color_active: String,
    #[serde(default = "default_color_today")]
    color_today: String,
    #[serde(default = "default_color_dim")]
    color_dim: String,
    #[serde(default = "default_start_y")]
    start_y: u32,
    #[serde(default)]
    stickers: Vec<StickerConfig>,
}

fn default_bg_type() -> String { "color".to_string() }
fn default_bg_color() -> String { "#000000".to_string() }
fn default_color_active() -> String { "#FFCC00".to_string() }
fn default_color_today() -> String { "#FFFFFF".to_string() }
fn default_color_dim() -> String { "#787878".to_string() }
fn default_start_y() -> u32 { 720 }

impl Default for WallpaperSettings {
    fn default() -> Self {
        Self {
            background_type: default_bg_type(),
            background_color: default_bg_color(),
            background_image: String::new(),
            color_active: default_color_active(),
            color_today: default_color_today(),
            color_dim: default_color_dim(),
            start_y: default_start_y(),
            stickers: Vec::new(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
struct ClassInfo {
    subject: String,
    room: String,
    time: String,
}

#[derive(Deserialize, Debug, Clone)]
struct DaySchedule {
    day: String,
    classes: Vec<ClassInfo>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum SchedulePayload {
    WithSettings {
        settings: WallpaperSettings,
        schedule: Vec<DaySchedule>,
    },
    LegacyArray(Vec<DaySchedule>),
}

const HEIGHT: u32 = 2400;
const WIDTH: u32 = 1080;

fn hex_to_rgba(hex: &str, fallback: Rgba<u8>) -> Rgba<u8> {
    let clean = hex.trim_start_matches('#');
    if clean.len() == 6 {
        let r = u8::from_str_radix(&clean[0..2], 16).unwrap_or(fallback.0[0]);
        let g = u8::from_str_radix(&clean[2..4], 16).unwrap_or(fallback.0[1]);
        let b = u8::from_str_radix(&clean[4..6], 16).unwrap_or(fallback.0[2]);
        return Rgba([r, g, b, 255]);
    }
    fallback
}

fn parse_time_to_minutes(time_str: &str) -> u32 {
    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() == 2 {
        let h: u32 = parts[0].parse().unwrap_or(0);
        let m: u32 = parts[1].parse().unwrap_or(0);
        return h * 60 + m;
    }
    0
}

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
    let config_path = if std::path::Path::new("config/schedule.json").exists() {
        "config/schedule.json"
    } else if std::path::Path::new("/data/adb/modules/schedule_wallpaper/system/etc/config/schedule.json").exists() {
        "/data/adb/modules/schedule_wallpaper/system/etc/config/schedule.json"
    } else {
        "schedule.json"
    };

    let json_data = fs::read_to_string(config_path).unwrap_or_else(|_| "[]".to_string());
    let payload: SchedulePayload = serde_json::from_str(&json_data).unwrap_or(SchedulePayload::LegacyArray(vec![]));

    let (settings, schedule) = match payload {
        SchedulePayload::WithSettings { settings, schedule } => (settings, schedule),
        SchedulePayload::LegacyArray(schedule) => (WallpaperSettings::default(), schedule),
    };

    let bg_color = hex_to_rgba(&settings.background_color, Rgba([0, 0, 0, 255]));
    let color_active = hex_to_rgba(&settings.color_active, Rgba([255, 204, 0, 255]));
    let color_today = hex_to_rgba(&settings.color_today, Rgba([255, 255, 255, 255]));
    let color_dim = hex_to_rgba(&settings.color_dim, Rgba([120, 120, 120, 255]));

    let config = Config::new(
        WIDTH,
        HEIGHT,
        bg_color,
        Some(BLACK),
        "jadwal_kuliah.png",
        Some("./assets/GeistMono-Medium.ttf"),
    );

    let mut image = Image::new();
    let mut components = Vec::new();

    let now = Local::now();
    let current_day = now.format("%A").to_string();
    let current_minutes = now.hour() * 60 + now.minute();

    let mut current_y = settings.start_y;

    for daily in schedule {
        let day_upper = daily.day.to_uppercase();
        let day_str: &'static str = Box::leak(day_upper.into_boxed_str());

        let is_today = daily.day.eq_ignore_ascii_case(&current_day);
        let header_color = if is_today { color_today } else { color_dim };

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
            let free_color = if is_today { color_today } else { color_dim };
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
                        color_active
                    } else {
                        color_today
                    }
                } else {
                    color_dim
                };

                components.push(Components::Text(
                    220,
                    current_y,
                    32,
                    subject_str,
                    text_color,
                    None,
                ));

                components.push(Components::Text(
                    560, current_y, 32, room_str, text_color, None,
                ));

                components.push(Components::Text(
                    840, current_y, 32, time_str, text_color, None,
                ));

                current_y += 42;
            }
            current_y += 25;
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
