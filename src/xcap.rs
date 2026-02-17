use fs_extra::dir;
use std::path::PathBuf;
use ::xcap::Monitor;
use chrono::Local;
use crate::region::select_region;
use image::{DynamicImage, ImageFormat};
use std::io::{Write, Cursor};
use std::process::{Command, Stdio};

fn monitor_data() -> (Monitor, (u32, u32)) {
    let monitors = Monitor::all().unwrap();
    let current = monitors
        .iter()
        .find(|m| m.is_primary().unwrap_or(false))
        .unwrap_or(&monitors[0]);

    let res = (current.width().expect("error"), current.height().expect(""));
    (current.clone(), res)
}

pub fn fullscreen_shot() {
    let date = Local::now();
    let datetime = date.format("%Y-%m-%d_%H-%M-%S").to_string();

    let (monitor, (w, h)) = monitor_data();
    let image = monitor.capture_region(0, 0, w, h).expect("capture_region failed");
    let dynamic_image = DynamicImage::ImageRgba8(image);

    let file_path = save_and_copy(dynamic_image, format!("screenshot_{}.png", datetime));
    println!("Screenshot saved to: {}", file_path.display());
    println!("Screenshot copied to clipboard");
}

pub fn region_screenshot() {
    let (monitor, (_w,_h)) = monitor_data();
    let date = Local::now();
    let datetime = date.format("%Y-%m-%d_%H-%M-%S").to_string();

    let (x, y, width, height) = select_region();
    let image = monitor.capture_region(x, y, width, height).expect("Error capturing region");
    let dynamic_image = DynamicImage::ImageRgba8(image);

    let file_path = save_and_copy(dynamic_image, format!("screenshot_{}.png", datetime));
    println!("Screenshot saved to: {}", file_path.display());
    println!("Screenshot copied to clipboard");
}

fn save_and_copy(image: DynamicImage, filename: String) -> PathBuf {
    let mut dir_path = dirs::home_dir().expect("Cannot determine home directory");
    dir_path.push("Pictures/Screenshots");
    dir::create_all(&dir_path, false).unwrap();

    let mut file_path = dir_path.clone();
    file_path.push(filename);

    image.save(&file_path).unwrap();
    clipboard(&image);

    file_path
}

pub fn clipboard(img: &DynamicImage) {
    let mut buf = Cursor::new(Vec::new());
    img.write_to(&mut buf, ImageFormat::Png).expect("Failed to write image to buffer");
    let png_bytes = buf.into_inner();

    let mut child = Command::new("wl-copy")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to spawn wl-copy");

    child.stdin.as_mut().unwrap().write_all(&png_bytes).expect("Failed to write PNG to wl-copy");
    child.wait().expect("wl-copy failed");
}

