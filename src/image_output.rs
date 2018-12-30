use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::color::*;

/// Writes an Image to a file with the Netpbm graphics format.
///
/// https://en.wikipedia.org/wiki/Netpbm_format
///
/// One can use `convert` from ImageMagick to make this something friendlier,
/// but Preview.app on macOS does view these just fine.
#[allow(clippy::ptr_arg)]
pub fn write_ppm(path: &Path, image: &Image) -> std::io::Result<()> {
    let width = image.len();
    let height = image[0].len();

    let mut file = match File::create(path) {
        Ok(f) => f,
        Err(x) => return Err(x),
    };
    let header = get_ppm_header(width, height);
    let result = file.write_all(header.as_bytes());
    if result.is_err() {
        return result;
    }

    for y in 0..height {
        for col in image.iter().take(width) {
            let buf = col[y].to_u8_array();
            let result = file.write_all(&buf);
            if result.is_err() {
                return result;
            }
        }
    }
    Ok(())
}

fn get_ppm_header(width: usize, height: usize) -> String {
    // P6 declares this as a binary RGB color image.
    format!(
        "P6\n{width} {height}\n255\n",
        width = width,
        height = height
    )
}
