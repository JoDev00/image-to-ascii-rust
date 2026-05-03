use opencv::highgui;
use opencv::imgproc;
use opencv::imgproc::InterpolationFlags;
use opencv::prelude::*;
use opencv::videoio;
use std::io::{self, Write};

const ASCII_BRIGHTNESS_VALUES: &str = "@@#S%?*+;:,";

// https://medium.com/@ekfqlwcjswl/using-opencv-in-rust-videocapture-d9e817f8e97c was a very helpful resource for this
fn main() -> opencv::Result<()> {
    let mut cap = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut frame = Mat::default();

    if !cap.is_opened()? {
        panic!("Failed to open video capture device");
    }

    loop {
        cap.read(&mut frame)?;

        let mut resized_frame = Mat::default();
        imgproc::resize(
            &frame,
            &mut resized_frame,
            opencv::core::Size::new(64, 64),
            0.0,
            0.0,
            InterpolationFlags::INTER_LINEAR as i32,
        )?;

        print!("\x1B[H");
        print!("{}", frame_to_ascii(&resized_frame)?);
        io::stdout().flush().unwrap();

        if highgui::wait_key(1)? == 27 {
            break;
        }
    }

    return Ok(());
}

fn frame_to_ascii(frame: &Mat) -> opencv::Result<String> {
    let mut ascii = String::new();

    for y in 0..frame.rows() {
        for x in 0..frame.cols() {
            let pixel = frame.at_2d::<opencv::core::Vec3b>(y, x)?;

            let rgba = pixel.0;
            let avg = (rgba[0] as u16 + rgba[1] as u16 + rgba[2] as u16) / 3;
            let char = get_ascii_character(avg, 255);

            ascii.push(' ');
            ascii.push_str(&get_colored_char(char, rgba[0], rgba[1], rgba[2]));
        }
        ascii.push('\n');
    }
    return Ok(ascii);
}

fn get_ascii_character(avg: u16, alpha: u8) -> char {
    if alpha == 0 {
        return ' ';
    }

    let index = (avg as usize) / (255 / ASCII_BRIGHTNESS_VALUES.len());
    return ASCII_BRIGHTNESS_VALUES.chars().nth(index).unwrap_or(' ');
}

fn get_colored_char(ch: char, r: u8, g: u8, b: u8) -> String {
    return format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, ch);
}
