#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::process::Command;
use std::path::Path;
use std::error::Error;
use std::sync::mpsc;
use std::thread;
use glob::glob;
use regex::Regex;
use xlsxwriter::*;
use tauri::{generate_handler, Builder, Manager};

fn check_ffprobe() -> bool {
    match Command::new("ffprobe").arg("-version").output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

fn get_metadata<R: Fn(&str)>(path: &str, log_fn: R) -> Result<Vec<(String, f64, f64)>, Box<dyn Error>> {
    let video_files = glob(&format!("{}/*.mp4", path))?
        .chain(glob(&format!("{}/*.MP4", path))?)
        .chain(glob(&format!("{}/*.avi", path))?)
        .chain(glob(&format!("{}/*.AVI", path))?)
        .chain(glob(&format!("{}/*.mov", path))?)
        .chain(glob(&format!("{}/*.MOV", path))?)
        .chain(glob(&format!("{}/*.mpg", path))?)
        .chain(glob(&format!("{}/*.MPG", path))?);

    let mut video_metadata = Vec::new();
    for file in video_files {
        let file_path = match file {
            Ok(path) => path,
            Err(e) => {
                log_fn(&format!("Error accessing file: {}", e));
                continue;
            }
        };

        let file_name = match file_path.file_name() {
            Some(name) => name.to_string_lossy(),
            None => {
                log_fn(&format!("Error accessing file name for {:?}", file_path));
                continue;
            }
        };
        if file_name.starts_with(".") {
            log_fn("Skipping hidden video file as it is likely a Linux or MacOS system file and not a real rush.");
            continue;
        }

        let file_path_str = file_path.to_str().ok_or("Invalid file path")?.to_string();

        let fps_output = Command::new("ffprobe")
            .arg("-v")
            .arg("error")
            .arg("-select_streams")
            .arg("v")
            .arg("-of")
            .arg("default=noprint_wrappers=1:nokey=1")
            .arg("-show_entries")
            .arg("stream=r_frame_rate")
            .arg(&file_path_str)
            .output();

        let fps_string = match fps_output {
            Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
            Err(e) => {
                log_fn(&format!("Error getting FPS for {}: {}", &file_path_str, e));
                continue;
            }
        };

        if fps_string.is_empty() {
            log_fn(&format!("Empty FPS string for {}", &file_path_str));
            continue;
        }

        log_fn(&format!("FPS String for {}: {}", &file_path_str, &fps_string));
        let fps: f64 = parse_fps(&fps_string)?;

        let duration_output = Command::new("ffprobe")
            .arg("-v")
            .arg("error")
            .arg("-select_streams")
            .arg("v")
            .arg("-of")
            .arg("default=noprint_wrappers=1:nokey=1")
            .arg("-show_entries")
            .arg("stream=duration")
            .arg(&file_path)
            .output()?;
        let duration_string = String::from_utf8_lossy(&duration_output.stdout).trim().to_string();
        let duration: f64 = parse_duration(&duration_string)?;

        video_metadata.push((file_path.to_string_lossy().to_string(), fps, duration));
    }

    Ok(video_metadata)
}

fn parse_fps(fps_string: &str) -> Result<f64, Box<dyn Error>> {
    let fps_parts: Vec<&str> = fps_string.split('/').collect();
    match fps_parts.len() {
        2 => {
            let numerator: f64 = fps_parts[0].trim().parse()?;
            let denominator: f64 = fps_parts[1].trim().parse()?;
            Ok(numerator / denominator)
        }
        _ => {
            println!("Unexpected FPS format: {}", fps_string);
            Err("Invalid FPS format".into())
        }
    }
}

fn parse_duration(duration_string: &str) -> Result<f64, Box<dyn Error>> {
    let duration: f64 = duration_string.trim().parse()?;
    Ok(duration)
}

fn timecode_to_frame(timecode: &str, fps: f64) -> i64 {
    let re = Regex::new(r"(\d+):(\d+):(\d+):(\d+)").unwrap();
    let captures = re.captures(timecode).unwrap();
    let h: i64 = captures[1].parse().unwrap();
    let m: i64 = captures[2].parse().unwrap();
    let s: i64 = captures[3].parse().unwrap();
    let f: i64 = captures[4].parse().unwrap();
    (h * 3600 + m * 60 + s) * (fps as i64) + f
}

fn frame_to_timecode(frame: i64, fps: f64) -> String {
    let total_seconds = frame / (fps as i64);
    let h = total_seconds / 3600;
    let m = (total_seconds / 60) % 60;
    let s = total_seconds % 60;
    let f = frame % (fps as i64);
    format!("{:02}:{:02}:{:02}:{:02}", h, m, s, f)
}

#[tauri::command]
fn generate_metadata(folder_path: String, start_timecode: String, output_file: String, app_handle: tauri::AppHandle) -> Result<(), String> {
    if !check_ffprobe() {
        return Err("FFmpeg is either not installed, or not in your PATH.".into());
    }

    let (tx, rx) = mpsc::channel();
    let window = app_handle.get_window("main").unwrap();

    // Start a new thread for metadata generation
    thread::spawn(move || {
        let log_fn = |message: &str| {
            tx.send(message.to_string()).unwrap();
        };

        match get_metadata(&folder_path, log_fn) {
            Ok(metadata) => {
                let workbook = Workbook::new(&format!("{}/{}.xlsx", folder_path, output_file))
                    .map_err(|e| format!("Error creating workbook: {}", e)).unwrap();
                let mut worksheet = workbook.add_worksheet(None)
                    .map_err(|e| format!("Error adding worksheet: {}", e)).unwrap();

                worksheet.write_string(0, 0, "Director", None).unwrap();
                worksheet.write_string(0, 1, "Producer", None).unwrap();
                worksheet.write_string(0, 2, "DOP", None).unwrap();
                worksheet.write_string(0, 3, "File", None).unwrap();
                worksheet.write_string(0, 4, "Roll", None).unwrap();
                worksheet.write_string(0, 5, "Scene", None).unwrap();
                worksheet.write_string(0, 6, "Slate", None).unwrap();
                worksheet.write_string(0, 7, "Take", None).unwrap();
                worksheet.write_string(0, 8, "Comments", None).unwrap();
                worksheet.write_string(0, 9, "Timecode In", None).unwrap();
                worksheet.write_string(0, 10, "Timecode Out", None).unwrap();
                worksheet.write_string(0, 11, "Usable", None).unwrap();

                let mut row = 1;
                let mut prev_timecode = start_timecode;
                let mut first_clip = true;

                for file_metadata in metadata {
                    let file = Path::new(&file_metadata.0);
                    let file_name = file.file_name().unwrap().to_str().unwrap();
                    let fps = file_metadata.1;
                    let duration = file_metadata.2;

                    let frame_in = timecode_to_frame(&prev_timecode, fps);
                    if !first_clip {
                        prev_timecode = frame_to_timecode(frame_in + 1, fps);
                    }

                    let frame_out = frame_in + ((duration * fps) as i64);
                    let timecode_out = frame_to_timecode(frame_out, fps);

                    worksheet.write_string(row, 3, &file_name, None).unwrap();
                    worksheet.write_string(row, 9, &prev_timecode, None).unwrap();
                    worksheet.write_string(row, 10, &timecode_out, None).unwrap();

                    prev_timecode = timecode_out.clone();
                    row += 1;

                    if first_clip {
                        first_clip = false;
                    }
                }

                workbook.close().map_err(|e| format!("Error closing workbook: {}", e)).unwrap();
                tx.send(format!("Output file saved to {}/{}.xlsx", folder_path, output_file)).unwrap();
            }
            Err(e) => {
                tx.send(format!("Error getting metadata: {}", e)).unwrap();
            }
        }
    });

    // Listen for messages from the thread
    thread::spawn(move || {
        for message in rx {
            window.emit("log-message", message).unwrap();
        }
    });

    Ok(())
}

fn main() {
    Builder::default()
        .invoke_handler(generate_handler![generate_metadata])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
