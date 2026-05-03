use clap::Parser;
use image::GenericImageView;
use std::str;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    img_name: String,
    width: u32,
    height: u32,
}

const ASCII_BRIGHTNESS_VALUES: &str = "@@#S%?*+;:,";

fn main() {
    let args = Args::parse();

    let img = image::open(format!("src/images/{}", args.img_name))
        .unwrap()
        .resize(
            args.width,
            args.height,
            image::imageops::FilterType::Triangle,
        );
    println!("dimensions {:?}", img.dimensions());

    for pixel in img.pixels() {
        let rgba = pixel.2;
        let avg = (rgba[0] as u16 + rgba[1] as u16 + rgba[2] as u16) / 3;
        let char = get_ascii_character(avg, rgba[3]);

        if pixel.0 % img.width() == 0 {
            println!();
        }
        print!(" {}", get_colored_char(char, rgba[0], rgba[1], rgba[2]));
    }
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
