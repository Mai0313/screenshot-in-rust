use scrap::{Capturer, Display};
use std::fs::File;
use std::io::ErrorKind::WouldBlock;
use std::path::Path;
use std::thread;
use std::time::{Duration, SystemTime};
use md5;
use image::{ImageBuffer, Rgba, ColorType};
use image::codecs::png::PngEncoder;
use image::ImageEncoder; // 新增導入

fn main() {
    loop {
        let output_path_folder = "outputs";
        let capture_percent = 1.0;

        let display = Display::primary().expect("Couldn't find primary display.");
        let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
        
        let (width, height) = (capturer.width(), capturer.height());
        let capture_width = (width as f32 * capture_percent) as u32;
        let capture_height = (height as f32 * capture_percent) as u32;

        let frame = loop {
            match capturer.frame() {
                Ok(f) => break f,
                Err(error) => {
                    if error.kind() != WouldBlock {
                        panic!("Capture error: {}", error);
                    }
                    thread::sleep(Duration::from_millis(1));
                }
            }
        };

        let filename = format!("{:x}.png", md5::compute(format!("{:?}", SystemTime::now())));
        let path = Path::new(output_path_folder).join(filename);
        let mut file = File::create(&path).expect("Couldn't create file.");

        let buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(capture_width, capture_height, frame.to_vec()).expect("Couldn't create image buffer.");
        
        // 使用新的方法保存圖像
        PngEncoder::new(&mut file).write_image(&buffer, capture_width, capture_height, ColorType::Rgba8).expect("Couldn't encode PNG.");

        println!("Screenshot saved to {} at {:?}", output_path_folder, SystemTime::now());
        thread::sleep(Duration::from_secs(1));
    }
}