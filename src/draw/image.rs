use std::path::Path;
use viuer::{Config, print_from_file};

pub fn draw_image(path: &Path, x: u16, y: u16, width: Option<u32>, height: Option<u32>) {
    let conf = Config {
        x,
        y: y as i16,
        width,
        height,
        ..Default::default()
    };
    print_from_file(path, &conf).expect("Failed to print image");
}
