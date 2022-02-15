use anyhow::Result;
use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use rscam::{Camera, Config};
use std::io::Cursor;

const ASCII_CHARSET: &str = "Ñ@#W$9876543210?!abc;:+=-,._                       ";

fn main() -> Result<()> {
    let mut camera = Camera::new("/dev/video0").unwrap();

    camera
        .start(&Config {
            interval: (1, 30), // 30 fps.
            format: b"MJPG",
            ..Default::default()
        })
        .unwrap();

    loop {
        let frame = camera.capture().unwrap();
        let ascii_charset_length = ASCII_CHARSET.chars().count();

        let img = ImageReader::new(Cursor::new(frame.to_vec()))
            .with_guessed_format()?
            .decode()?
            .resize_exact(64 * 3, 64, FilterType::Triangle)
            .grayscale();
        // println!("{:?}", img);
        print!("\x1B[2J\x1B[1;1H");
        let img_rgb = img.as_luma8().unwrap();
        for (x, _y, pixel) in img_rgb.enumerate_pixels() {
            if x == 0 {
                print!("\n");
            }
            print!(
                "{}",
                ASCII_CHARSET
                    .chars()
                    .nth(((pixel.0[0] as f32 / 256.0) * ascii_charset_length as f32) as usize)
                    .unwrap()
            );
        }
    }
}
