// use scrap::{Capturer, Display};
// use std::fs::{self, File};
// use std::io::ErrorKind::WouldBlock;
// use std::path::Path;
// use std::thread;
// use std::time::{Duration, SystemTime};
// use md5;
// use image::{ImageBuffer, Rgba, ColorType};
// use image::codecs::png::PngEncoder;
// use image::ImageEncoder;
// use chrono::Local;

// fn main() {
//     loop {
//         let output_path_folder = "outputs";
//         let capture_percent = 0.5; // 修改爲 50%

//         fs::create_dir_all(output_path_folder).expect("Couldn't create output folder.");

//         let display = Display::primary().expect("Couldn't find primary display.");
//         let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");

//         let (width, height) = (capturer.width(), capturer.height());
//         let capture_width = (width as f32 * capture_percent) as usize; // 轉換為 usize
//         let capture_height = (height as f32 * capture_percent) as usize; // 轉換為 usize

//         let frame = loop {
//             match capturer.frame() {
//                 Ok(f) => break f,
//                 Err(error) => {
//                     if error.kind() != WouldBlock {
//                         panic!("Capture error: {}", error);
//                     }
//                     thread::sleep(Duration::from_millis(1));
//                 }
//             }
//         };

//         // 計算中心區域的起始點
//         let start_x = (width - capture_width) / 2;
//         let start_y = (height - capture_height) / 2;

//         // 創建一個新的 buffer 用於儲存中心區域的像素
//         let mut buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(capture_width as u32, capture_height as u32);

//         for y in 0..capture_height {
//             for x in 0..capture_width {
//                 let index = 4 * ((start_y + y) * width + (start_x + x)); // 使用 start_x 和 start_y 來定位像素
//                 let pixel = Rgba([
//                     frame[index],     // R
//                     frame[index + 1], // G
//                     frame[index + 2], // B
//                     frame[index + 3], // A
//                 ]);
//                 buffer.put_pixel(x as u32, y as u32, pixel);
//             }
//         }

//         let filename = format!("{:x}.png", md5::compute(format!("{:?}", SystemTime::now())));
//         let path = Path::new(output_path_folder).join(filename);
//         let mut file = File::create(&path).expect("Couldn't create file.");

//         PngEncoder::new(&mut file).write_image(&buffer, capture_width as u32, capture_height as u32, ColorType::Rgba8).expect("Couldn't encode PNG.");

//         let now = Local::now();
//         println!("Screenshot saved to {} at {}", output_path_folder, now.format("%Y-%m-%d %H:%M:%S"));
//         thread::sleep(Duration::from_secs(1));
//     }
// }

use scrap::{Capturer, Display};
use std::fs::{self, File};
use std::io::ErrorKind::WouldBlock;
use std::path::Path;
use std::thread;
use std::time::{Duration, SystemTime};
use md5;
use image::{ImageBuffer, Rgba, ColorType};
use image::codecs::png::PngEncoder;
use image::ImageEncoder;
use chrono::Local;

fn main() {
    loop {
        let output_path_folder = "outputs";
        let capture_percent = 1.0;

        fs::create_dir_all(output_path_folder).expect("Couldn't create output folder.");

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

        PngEncoder::new(&mut file).write_image(&buffer, capture_width, capture_height, ColorType::Rgba8).expect("Couldn't encode PNG.");

        let now = Local::now();
        println!("Screenshot saved to {} at {}", output_path_folder, now.format("%Y-%m-%d %H:%M:%S"));
        thread::sleep(Duration::from_secs(1));
    }
}
