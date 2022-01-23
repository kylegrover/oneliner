use std::fs::File;

use circles_drawing::{
    canny_devernay::canny_devernay, connect_pathes, path_length, utils::*, write_pathes_as_svg,
};
use image::{codecs::pnm::PnmDecoder, ColorType, ImageDecoder};

fn main() {
    let args = std::env::args().into_iter().take(3).collect::<Vec<_>>();
    let (input_path, output_prefix) = (&args[1], &args[2]);
    let input_path = std::path::Path::new(input_path);
    let input_file_name = input_path.file_name().unwrap().to_str().unwrap();

    let image_file = File::open(input_path).unwrap();
    let decoder = PnmDecoder::new(image_file).unwrap();
    let (width, height) = decoder.dimensions();
    let color_type = decoder.color_type();
    let mut image_buf = vec![0; decoder.total_bytes() as usize];
    decoder.read_image(image_buf.as_mut()).unwrap();

    let image_gray = match color_type {
        ColorType::Rgb8 => image_buf
            .chunks(3)
            .map(|rgb| rgb_to_grayscale(rgb[0], rgb[1], rgb[2]))
            .collect::<Vec<_>>(),
        ColorType::L8 => image_buf,
        _ => panic!("unsupported color type {:?}", color_type),
    };

    let params = vec![
        (0, 0, 0),
        (1, 0, 0),
        (2, 0, 0),
        (1, 5, 5),
        (1, 10, 10),
        (1, 15, 15),
        (1, 20, 20),
        (1, 5, 10),
        (1, 5, 15),
        (1, 5, 20),
        (1, 0, 15),
        (1, 1, 15),
        (1, 3, 15),
        (1, 10, 15),
    ];

    for (s, l, h) in params {
        println!("S = {}, L = {}, H = {}", s, l, h);

        let pathes = canny_devernay(
            &image_gray,
            height as usize,
            width as usize,
            s as f64,
            h as f64,
            l as f64,
        );

        // let t = std::time::Instant::now();
        // let epsilon = (width as f64) * 0.02;

        // let kepts = pathes.iter().map(|path| {
        //     let mut kept = vec![false; path.len()];
        //     ramer_douglas_peucker(path, epsilon, &mut kept);
        //     kept
        // });

        // let mut simplified_pathes = vec![];

        // for (kept, path) in kepts.zip(pathes.iter()) {
        //     let simplified_path = kept
        //         .into_iter()
        //         .zip(path)
        //         .filter(|&(k, _)| k)
        //         .map(|(_, p)| *p)
        //         .collect::<Vec<_>>();
        //     simplified_pathes.push(simplified_path);
        // }

        // let pathes = simplified_pathes;
        // println!("simplifying pathes took {:?}", t.elapsed());

        // println!("pathes.len() = {}", pathes.len());

        // let output_path = std::path::Path::new(output_prefix)
        //     .join(format!("{}_{}_{}_{}.svg", input_file_name, s, l, h));

        // write_pathes_as_svg(
        //     std::io::BufWriter::new(File::create(output_path).unwrap()),
        //     &pathes,
        //     height as usize,
        //     width as usize,
        // )
        // .unwrap();

        let mut pathes = pathes;
        pathes.sort_by(|a, b| path_length(b).partial_cmp(&path_length(a)).unwrap());
        pathes.truncate(2000);
        pathes.extend(connect_pathes(&pathes));

        let output_path = std::path::Path::new(output_prefix).join(format!(
            "{}_{}_{}_{}_2k_hull_connected.svg",
            input_file_name, s, l, h
        ));

        let num_points = pathes.iter().map(|p| p.len()).sum::<usize>();
        println!("num points = {}", num_points);

        write_pathes_as_svg(
            std::io::BufWriter::new(File::create(output_path).unwrap()),
            &pathes,
            height as usize,
            width as usize,
        )
        .unwrap();
    }
}
