use std::fs::File;
use std::io::prelude::*;

use crate::vector::*;

pub fn write_ppm(image: &Vec<Vec<Vec3f>>) -> std::io::Result<()> {
    let mut file = File::create("foo.ppm")?;
    write!(
        file,
        "P6\n{width} {height}\n255\n",
        width = 100,
        height = 80
    )?;

    for y in 0..80 {
        for x in 0..100 {
            let buf = [
                (image[x][y].x * 255.0) as u8,
                (image[x][y].y * 255.0) as u8,
                (image[x][y].z * 255.0) as u8,
            ];
            file.write_all(&buf);
        }
    }
    Ok(())
}
