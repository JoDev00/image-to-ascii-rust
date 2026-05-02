use image::GenericImageView;
use std::str;

const ASCII_BRIGHTNESS_VALUES: &str = "@@#S%?*+;:,";
const SIZE: (u32, u32) = (64, 64);

fn get_ascii_character(avg: u16, alpha: u8) -> char {
    if alpha == 0 {
        return ' ';
    }

    let index = (avg as usize) / (255 / ASCII_BRIGHTNESS_VALUES.len());
    return ASCII_BRIGHTNESS_VALUES.chars().nth(index).unwrap_or(' ');
}

fn main() {
    let img_name = "smiley.png";
    let img = image::open(format!("src/images/{}", img_name))
        .unwrap()
        .resize(SIZE.0, SIZE.1, image::imageops::FilterType::Triangle);
    println!("dimensions {:?}", img.dimensions());

    for pixel in img.pixels() {
        let rgba = pixel.2;
        let avg = (rgba[0] as u16 + rgba[1] as u16 + rgba[2] as u16) / 3;
        let char = get_ascii_character(avg, rgba[3]);

        if pixel.0 % img.width() == 0 {
            println!();
        }
        print!(" {}", char);
    }
}
