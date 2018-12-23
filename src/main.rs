mod vector;

use crate::vector::*;

fn main() {
    let u: Vec3f = Vec3 {
        x: 0f64,
        y: 1f64,
        z: 2f64,
    };
    let mut v: Vec3f = Vec3 {
        x: 1f64,
        y: 2f64,
        z: 3f64,
    };

    println!("u + v = {:?}", u + v);
    println!("u - v = {:?}", u - v);
    println!("u * v = {:?}", u * v);
    println!("3.0 * v = {:?}", v * 3f64);
    println!("||v|| = {:?}", v.norm());
    v.normalize();
    println!("after normalize: ||v|| = {:?}", v.norm());
}
