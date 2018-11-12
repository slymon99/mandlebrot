extern crate image;

use std::fs::File;
use std::io::prelude::*;

const MAX_COUNT: i32 = 1_000;

fn main() {
    image_mandlebrot(-3.0, 1.5, -1.0, 1.0, 1920, 1080);
//    assert_eq!(false, in_set(5.0, 5.4));
//    assert_eq!(true, in_set(0.0, 0.0));
//    assert_eq!(true, in_set(0.0, -0.5));
//    assert_eq!(true, in_set(0.0, -0.7));
//    assert_eq!(false, in_set(0.0, 1.0));
}

fn text_mandlebrot(x_min: f64, x_max: f64, y_min: f64, y_max: f64, w: i32, h: i32){
    let x_dist = x_max - x_min;
    let y_dist = y_max - y_min;

    let mut file = File::create(format!("out/text/mandlebrot {}x{}.txt", w, h))
        .expect("file creation broke");
    for row in 0..h {
        let mut line = String::new();
        for col in 0..w {
            match in_set(x_min + x_dist * col as f64 / w as f64,
                         y_min + y_dist * row as f64 / h as f64){
                true => line.push('*'),
                false => line.push(' ')
            };
        }
        line.push('\n');
        file.write_all(line.as_bytes()).expect("Unable to write");
    }
}

fn image_mandlebrot(x_min: f64, x_max: f64, y_min: f64, y_max: f64, w: u32, h: u32){
    let x_dist = x_max - x_min;
    let y_dist = y_max - y_min;

    let mut imgbuf = image::GrayImage::new(w, h);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut(){
        match in_set(x_min + x_dist * x as f64 / w as f64,
                     y_min + y_dist * y as f64 / h as f64){
            true => *pixel = image::Luma([0]),
            false => *pixel = image::Luma([255])
        };
    }

    imgbuf.enumerate_pixels_mut().for_each

    imgbuf.save("fractal.png").unwrap();
}

fn in_set(a: f64, b: f64) -> bool {
    let mut a_mut = a;
    let mut b_mut = b;

    for _ in 0..MAX_COUNT{
        let a_tmp = (a_mut * a_mut - b_mut * b_mut) + a;
        b_mut = 2.0 * a_mut * b_mut + b;
        a_mut = a_tmp;
        if a_mut * a_mut + b_mut * b_mut >= 4.0{
            return false
        }
    }
    true
}