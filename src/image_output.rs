use std::fs::File;
use std::io::prelude::*;

use crate::color::*;

pub fn write_ppm(image: &Image) -> std::io::Result<()> {
    let width = image.len();
    let height = image[0].len();

    let mut file = match File::create("foo.ppm") {
        Ok(f) => f,
        Err(x) => return Err(x),
    };
    let header = get_ppm_header(width, height);
    let result = file.write_all(header.as_bytes());
    if result.is_err() {
        return result;
    }

    for y in 0..height {
        for x in 0..width {
            let buf = image[x][y].to_u8_array();
            let result = file.write_all(&buf);
            if result.is_err() {
                return result;
            }
        }
    }
    Ok(())
}

fn get_ppm_header(width: usize, height: usize) -> String {
    format!(
        "P6\n{width} {height}\n255\n",
        width = width,
        height = height
    )
}
