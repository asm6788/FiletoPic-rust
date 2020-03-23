extern crate stopwatch;
extern crate image;

use std::io;
use std::io::BufReader;
use std::fs::File;
use std::io::Read;
use std::fs;
use std::env;
use stopwatch::{Stopwatch};


enum PictureShape
{
    Square,
    Rectangular,
    Circle
}

struct PictureInfo {
    width: u32,
    height: u32,
    shape: PictureShape,
}

struct Divisor {
    A: u32,
    B: u32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let file_size = fs::metadata(file_name).unwrap().len();
    run_single(file_name ,file_size,calculate_size(file_size, PictureShape::Square));
}

fn calculate_size(file_size : u64, shape : PictureShape) -> PictureInfo {
    match shape {
        PictureShape::Square => {
            let mut length = ((file_size / 3) as f64).sqrt();

            if length % 1.0 != 0.0 {
                length = length.ceil()
            }

            PictureInfo {
                width: length as u32,
                height: length as u32,
                shape: PictureShape::Square,
            }
        }
        PictureShape::Rectangular => {
            let mut length = ((file_size / 3) as f64).sqrt();

            if (length / 3.0) % 1.0 != 0.0 {
                length = length.ceil()
            }
            let divisors = get_divisors(length as u32);
            for i in 0..divisors.len() {
                println!("{}: w: {} h:{}",i,divisors[i].A,divisors[i].B);
            }

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            PictureInfo {
                width: divisors[input.trim().parse::<usize>().unwrap()].A,
                height: divisors[input.trim().parse::<usize>().unwrap()].B,
                shape: PictureShape::Rectangular,
            }
        }
        _ =>  PictureInfo {
            width: 0,
            height: 0,
            shape: PictureShape::Rectangular,
        }
    }
}

fn get_divisors(num : u32) ->  Vec<Divisor>{
    let mut temp = Vec::new();
    for i in 1..num {
        if num % i == 0
        {
            temp.push(Divisor{A:i, B:num / i});
        }
    }
    return temp;
}

fn run_single(file_path : &str, file_size : u64, size : PictureInfo){
    let sw = Stopwatch::start_new();
    let mut offset = 0;
    let mut f = BufReader::new(File::open(file_path).unwrap());
    let mut file_byte = [0; 3];
    let mut imgbuf = image::ImageBuffer::new(size.width, size.height); 

'outer: for x in 0..size.width{
    for y in 0..size.height{
        match size.shape {
            PictureShape::Square => {
                f.read(&mut file_byte[..]).unwrap();
                if offset >= file_size
                {
                 
                    imgbuf.put_pixel(x, y, image::Rgba([0, 0, 0, 3]));
                    break;
                }
                else if offset + 2 == file_size
                {
          
                    imgbuf.put_pixel(x, y, image::Rgba([file_byte[0], file_byte[1], 0, 2]));
                    break 'outer;
                }
                else if offset + 1 == file_size
                {
     
                    imgbuf.put_pixel(x, y, image::Rgba([file_byte[0], 0, 0, 1]));
                }
                else if file_size <= offset
                {
                    break 'outer;
                }
                else
                {
    
                    imgbuf.put_pixel(x, y, image::Rgba([file_byte[0], file_byte[1], file_byte[2], 255]));
                }
                offset += 3;
            }

            _ => {}
        }
    }
}

println!("PNG 압축시작");
imgbuf.save("output.png").unwrap();
println!("작업완료! {}ms", sw.elapsed_ms());
}