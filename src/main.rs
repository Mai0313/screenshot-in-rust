use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;
use std::time::Duration;
use std::thread::sleep;
use image::jpeg::JPEGEncoder;
use image::ColorType;
use std::fs::File;
use std::path::Path;

fn main() {
    // 獲取主螢幕
    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());

    loop {
        // 等待直到我們有一個螢幕快照
        let buffer = loop {
            match capturer.frame() {
                Ok(buffer) => break buffer,
                Err(error) => {
                    if error.kind() == WouldBlock {
                        // 等待一會兒再試
                        sleep(Duration::from_millis(100));
                        continue;
                    } else {
                        // 其他錯誤
                        panic!("Error: {}", error);
                    }
                }
            };
        };

        // 獲取當前時間並格式化為檔案名稱
        let filename = format!("outputs/{}.jpg", chrono::Local::now().format("%Y%m%d%H%M%S"));

        // 將快照保存到檔案
        let output_file = File::create(&Path::new(&filename)).unwrap();
        let mut encoder = JPEGEncoder::new_with_quality(output_file, 80);
        encoder.encode(&buffer, w as u32, h as u32, ColorType::RGBA(8)).unwrap();

        println!("Screenshot saved to {}", filename);

        // 等待一秒
        sleep(Duration::from_secs(1));
    }
}