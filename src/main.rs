use std::fs::{File, create_dir_all};
use std::path::Path;

use image::{io::Reader as ImageReader, GenericImageView, GrayImage, RgbImage};
use image::ColorType;
use oneliner::{canny_devernay::Params, image_to_cycle, utils::*, write_pathes_as_svg};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input_path> <output_prefix>", args[0]);
        std::process::exit(1);
    }

    let (input_path, output_prefix) = (&args[1], &args[2]);
    let input_path = std::path::Path::new(input_path);
    let input_file_name = input_path.file_name().unwrap().to_str().unwrap();

    let img = ImageReader::open(&input_path).unwrap().decode().unwrap();

    let (width, height) = img.dimensions();
    let color_type = img.color();
    println!("ColorType: {:?}", color_type);
    
    let image_gray = match color_type {
        ColorType::Rgb8 => {
            let rgb_image: RgbImage = img.to_rgb8();
            rgb_image.into_raw().chunks(3).map(|rgb| rgb_to_grayscale(rgb[0], rgb[1], rgb[2])).collect::<Vec<_>>()
        },
        ColorType::L8 => {
            let gray_image: GrayImage = img.to_luma8();
            gray_image.into_raw()
        },
        _ => panic!("unsupported color type {:?}", color_type),
    };

    let params = vec![
        (1, 5, 20),
    ];

    let num_pathes = 300;

    for (s, l, h) in params {
        println!("S = {}, L = {}, H = {}", s, l, h);

        let final_path = image_to_cycle(
            &image_gray,
            height as usize,
            width as usize,
            Params {
                s: s as f64,
                l: l as f64,
                h: h as f64,
            },
            num_pathes,
        );

        println!("num points = {}", final_path.len());

        let file_path = format!("{}_{}_{}_{}_{}_hull_simplified_connected_eulerian_2.svg",
            input_file_name, s, l, h, num_pathes);
        let svg_output_file = Path::new(output_prefix).join(file_path);
        
        // Create the output directory if it doesn't already exist.
        if let Some(parent) = svg_output_file.parent() {
           create_dir_all(parent).expect("Failed to create directory");
        }

        write_pathes_as_svg(std::io::BufWriter::new(File::create(svg_output_file).unwrap()), &[&final_path],
            height as usize, width as usize).unwrap();
    }
}