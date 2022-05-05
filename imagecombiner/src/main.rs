mod args;
use args::Args;
use image::{io::*, DynamicImage, ImageFormat, imageops::FilterType::Triangle, GenericImageView};

#[derive(Debug)]
enum ImageDataError {
    DifferentFormat,
}

fn main() -> Result<(), ImageDataError> {
    let arguments = Args::new();

    let (img1, img1_format) = find_img(arguments.img1);
    let (img2, img2_format) = find_img(arguments.img2);

    if (img1_format != img2_format) {
        return Err(ImageDataError::DifferentFormat);
    }

    let (img1, img2) = resize_smallest(img1, img2);

    return Ok(());
}

fn find_img(path: String) -> (DynamicImage, ImageFormat) {
    let fd = Reader::open(path).unwrap();
    let format = fd.format().unwrap();
    let image = fd.decode().unwrap();
    return (image, format);
}

fn get_smallest_dim(d1: (u32, u32), d2: (u32, u32)) -> (u32, u32) {
    if d1.0 * d1.1 < d2.0 * d2.1 {
        return d1;
    } else {
        return d2;
    }
}

fn resize_smallest(img1: DynamicImage, img2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (w, h) = get_smallest_dim(img1.dimensions(), img2.dimensions());

    println!("{:?}", (w, h));

    if (w, h) == img1.dimensions() {
        img2.resize_exact(w, h, Triangle);
    } else {
        img2.resize_exact(w, h, Triangle);
    }
    
    return (img1, img2);
}