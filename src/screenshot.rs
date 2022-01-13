use image::ImageBuffer;
use scrap::{Capturer, Display};
use std::any::Any;
use std::fmt::Result;
use std::io::ErrorKind::WouldBlock;
use std::path::Path;
use std::thread::JoinHandle;
use std::{thread, time::Duration};

pub fn take<Q>(path: Q)
where
    Q: AsRef<Path>,
{
    let mut capturer = Capturer::new(Display::primary().unwrap()).unwrap();
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;

    let (w, h) = (capturer.width(), capturer.height());

    loop {
        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(error) => {
                if error.kind() == WouldBlock {
                    // Keep spinning.
                    thread::sleep(one_frame);
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        };

        println!("Captured! Saving...");

        // Flip the ARGB image into a BGRA image.

        let stride = buffer.len() / h;

        let image = ImageBuffer::from_fn((w as u32)/2, (h as u32)/2, |x, y| {
            let i = (stride * (y as usize) + 4 * (x as usize))*2;
            let r = buffer[i + 2];
            let g = buffer[i + 1];
            let b = buffer[i];
            image::Rgb([r, g, b])
        });

        // Save the image.

        image.save(path).unwrap();

        println!("Image saved to `screenshot.png`.");
        break;
    }
}