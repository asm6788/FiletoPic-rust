extern crate image;

use std::env;
use std::fs::{self, File};
use std::io::{self, Read};
use std::time::Instant;

enum PictureShape {
    Square,
    Rectangular,
    Circle,
}

struct PictureInfo {
    width: u32,
    height: u32,
    shape: PictureShape,
}

type Divisor = (u32, u32);

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let file_size = fs::metadata(file_name).unwrap().len();
    run_single(
        file_name,
        file_size,
        calculate_size(file_size, PictureShape::Square),
    );
}

fn calculate_size(file_size: u64, shape: PictureShape) -> PictureInfo {
    match shape {
        PictureShape::Square => {
            let length = ((file_size / 3) as f64).sqrt().ceil() as u32;

            PictureInfo {
                width: length,
                height: length,
                shape: PictureShape::Square,
            }
        }
        PictureShape::Rectangular => {
            let length = ((file_size / 3) as f64).sqrt() as u32;

            let divisors = get_divisors(length);
            for (i, divisor) in divisors.iter().enumerate() {
                println!("{}: w: {} h:{}", i, divisor.0, divisor.1);
            }

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let size = input.trim().parse::<usize>().unwrap();

            PictureInfo {
                width: divisors[size].0,
                height: divisors[size].1,
                shape: PictureShape::Rectangular,
            }
        }
        _ => PictureInfo {
            width: 0,
            height: 0,
            shape: PictureShape::Rectangular,
        },
    }
}

fn get_divisors(num: u32) -> Vec<Divisor> {
    let mut temp = Vec::new();
    // a * b = num이라면 b * a = num이다.
    for i in 1..=((num as f32).sqrt() as u32) {
        if num % i == 0 {
            temp.push((i, num / i));
            temp.push((num / i , i));
        }
    }
    return temp;
}

fn run_single(file_path: &str, file_size: u64, size: PictureInfo) {
    let start_time = Instant::now();
    let mut offset = 0;
    let mut f = File::open(file_path).unwrap();
    let mut file_byte = [0; 3];
    let mut imgbuf = image::ImageBuffer::new(size.width, size.height);

    'outer: for x in 0..size.width {
        for y in 0..size.height {
            match size.shape {
                PictureShape::Square => {
                    f.read(&mut file_byte[..]).unwrap();
                    if offset >= file_size {
                        imgbuf.put_pixel(x, y, image::Rgba([0, 0, 0, 3]));
                        break;
                    } else if offset + 2 == file_size {
                        imgbuf.put_pixel(x, y, image::Rgba([file_byte[0], file_byte[1], 0, 2]));
                        break 'outer;
                    } else if offset + 1 == file_size {
                        imgbuf.put_pixel(x, y, image::Rgba([file_byte[0], 0, 0, 1]));
                    } else if file_size <= offset {
                        break 'outer;
                    } else {
                        imgbuf.put_pixel(
                            x,
                            y,
                            image::Rgba([file_byte[0], file_byte[1], file_byte[2], 255]),
                        );
                    }
                    offset += 3;
                }

                _ => {}
            }
        }
    }

    println!("PNG 압축시작");
    imgbuf.save("output.png").unwrap();
    println!("작업완료! {:#?}", start_time.elapsed());
}
