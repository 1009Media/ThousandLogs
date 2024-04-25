use std::process::Command;
use std::path::Path;
use std::io::{self, Write};
use std::error::Error;

use glob::glob;
use regex::Regex;
use xlsxwriter::*;

// function to get video metadata
fn get_metadata(path: &str) -> Result<Vec<(String, f64, f64)>, Box<dyn Error>> {
    // scan for video files
    let video_files = glob(&format!("{}/*.mp4", path))?
        .chain(glob(&format!("{}/*.MP4", path))?)
        .chain(glob(&format!("{}/*.avi", path))?)
        .chain(glob(&format!("{}/*.AVI", path))?)
        .chain(glob(&format!("{}/*.mov", path))?)
        .chain(glob(&format!("{}/*.MOV", path))?)
        .chain(glob(&format!("{}/*.mpg", path))?)
        .chain(glob(&format!("{}/*.MPG", path))?);

    // Sort files by creation time
    let mut video_metadata = Vec::new();
    for file in video_files {
        let file_path = match file {
            Ok(path) => path,
            Err(e) => {
                eprintln!("Error accessing file: {}", e);
                continue; // Skip to the next file
            }
        };

        // Skip files with names starting with a dot
        let file_name = match file_path.file_name() {
            Some(name) => name.to_string_lossy(),
            None => {
                eprintln!("Error accessing file name for {:?}", file_path);
                continue; // Skip to the next file
            }
        };
        if file_name.starts_with(".") {
            println!("Skipping hidden video file as it is likely a Linux or MacOS system file and not a real rush.");
            continue; // Skip to the next file
        }

        let file_path_str = file_path.to_str().ok_or("Invalid file path")?.to_string();

        // get video metadata
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
                eprintln!("Error getting FPS for {}: {}", &file_path_str, e);
                continue; // Skip to the next file
            }
        };

        if fps_string.is_empty() {
            eprintln!("Empty FPS string for {}", &file_path_str);
            continue; // Skip to the next file
        }

        println!("FPS String for {}: {}", &file_path_str, &fps_string);
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
            // Print the FPS string for debugging
            println!("Unexpected FPS format: {}", fps_string);
            Err("Invalid FPS format".into())
        }
    }
}

// function to parse duration from string
fn parse_duration(duration_string: &str) -> Result<f64, Box<dyn Error>> {
    let duration: f64 = duration_string.trim().parse()?;
    Ok(duration)
}

// function to convert timecode string to frame number
fn timecode_to_frame(timecode: &str, fps: f64) -> i64 {
    let re = Regex::new(r"(\d+):(\d+):(\d+):(\d+)").unwrap();
    let captures = re.captures(timecode).unwrap();
    let h: i64 = captures[1].parse().unwrap();
    let m: i64 = captures[2].parse().unwrap();
    let s: i64 = captures[3].parse().unwrap();
    let f: i64 = captures[4].parse().unwrap();
    (h * 3600 + m * 60 + s) * (fps as i64) + f
}

// function to convert frame number to timecode string
fn frame_to_timecode(frame: i64, fps: f64) -> String {
    let total_seconds = frame / (fps as i64);
    let h = total_seconds / 3600;
    let m = (total_seconds / 60) % 60;
    let s = total_seconds % 60;
    let f = frame % (fps as i64);
    format!("{:02}:{:02}:{:02}:{:02}", h, m, s, f)
}

fn main() -> Result<(), Box<dyn Error>> {
    // display ascii art logo
    let ascii_logo = r#"
    █████╗ ██╗   ██╗████████╗ ██████╗ ████████╗ ██████╗██╗      ██████╗  ██████╗       ██████╗ ███████╗
   ██╔══██╗██║   ██║╚══██╔══╝██╔═══██╗╚══██╔══╝██╔════╝██║     ██╔═══██╗██╔════╝       ██╔══██╗██╔════╝
   ███████║██║   ██║   ██║   ██║   ██║   ██║   ██║     ██║     ██║   ██║██║  ███╗█████╗██████╔╝███████╗
   ██╔══██║██║   ██║   ██║   ██║   ██║   ██║   ██║     ██║     ██║   ██║██║   ██║╚════╝██╔══██╗╚════██║
   ██║  ██║╚██████╔╝   ██║   ╚██████╔╝   ██║   ╚██████╗███████╗╚██████╔╝╚██████╔╝      ██║  ██║███████║
   ╚═╝  ╚═╝ ╚═════╝    ╚═╝    ╚═════╝    ╚═╝    ╚═════╝╚══════╝ ╚═════╝  ╚═════╝       ╚═╝  ╚═╝╚══════╝

    "#;

    println!("{}", ascii_logo);

    // display version number
    println!("Version 2.0.1\n");

    // get folder path from user input
    print!("Enter folder path: ");
    io::stdout().flush()?;
    let mut path = String::new();
    io::stdin().read_line(&mut path)?;
    let path = path.trim();

    println!("Scanning folder, please wait...");

    // get video metadata
    let metadata = get_metadata(path)?;

    // get start timecode from user input
    print!("Enter start timecode (HH:MM:SS:FF). Press enter for 00:00:00:00: ");
    io::stdout().flush()?;
    let mut start_timecode = String::new();
    io::stdin().read_line(&mut start_timecode)?;
    let start_timecode = start_timecode.trim();
    let start_timecode = if start_timecode.is_empty() { "00:00:00:00" } else { start_timecode };

    // ask user for output file name and location
    println!("Where would you like to save the output excel file?");
    print!("Enter a file path, or leave blank to save in the same location as the video files: ");
    io::stdout().flush()?;
    let mut output_path = String::new();
    io::stdin().read_line(&mut output_path)?;
    let output_path = output_path.trim();
    let output_path = if output_path.is_empty() { path } else { output_path };

    print!("Enter a file name for the output excel file (excluding file extension): ");
    io::stdout().flush()?;
    let mut output_file = String::new();
    io::stdin().read_line(&mut output_file)?;
    let output_file = output_file.trim();

    // create Excel workbook
    let workbook = Workbook::new(&format!("{}/{}.xlsx", output_path, output_file))?;
    let mut worksheet = workbook.add_worksheet(None)?;

    // write headers
    worksheet.write_string(0, 0, "Director", None)?;
    worksheet.write_string(0, 1, "Producer", None)?;
    worksheet.write_string(0, 2, "DOP", None)?;
    worksheet.write_string(0, 3, "File", None)?;
    worksheet.write_string(0, 4, "Roll", None)?;
    worksheet.write_string(0, 5, "Scene", None)?;
    worksheet.write_string(0, 6, "Slate", None)?;
    worksheet.write_string(0, 7, "Take", None)?;
    worksheet.write_string(0, 8, "Comments", None)?;
    worksheet.write_string(0, 9, "Timecode In", None)?;
    worksheet.write_string(0, 10, "Timecode Out", None)?;
    worksheet.write_string(0, 11, "Usable", None)?;

    let mut row = 1; // start from second row
    let mut prev_timecode = start_timecode.to_string();
    let mut first_clip = true; // Flag to track the first clip

    for file_metadata in metadata {
        let file = Path::new(&file_metadata.0);
        let file_name = file.file_name().unwrap().to_str().unwrap();
        let fps = file_metadata.1;
        let duration = file_metadata.2;

        // calculate timecode in and timecode out
        let frame_in = timecode_to_frame(&prev_timecode, fps);

        // Adjust timecode in by one frame for all clips except the first one
        if !first_clip {
            prev_timecode = frame_to_timecode(frame_in + 1, fps); // Add one frame
        }

        let frame_out = frame_in + ((duration * fps) as i64);
        let timecode_out = frame_to_timecode(frame_out, fps);

        // write data to worksheet
        worksheet.write_string(row, 3, &file_name, None)?;
        worksheet.write_string(row, 9, &prev_timecode, None)?;
        worksheet.write_string(row, 10, &timecode_out, None)?;

        // Update prev_timecode to be timecode_out for the next iteration
        prev_timecode = timecode_out.clone();
        row += 1;

        // Update first_clip flag after processing the first clip
        if first_clip {
            first_clip = false;
        }
    }


    // close workbook
    workbook.close()?;

    println!("Output file saved to {}/{}.xlsx", output_path, output_file);
    Ok(())
}
