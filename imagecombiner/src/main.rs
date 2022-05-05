mod args;
use args::Args;
use image::{io::*, DynamicImage, ImageFormat};

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

    return Ok(());
}

fn find_img(path: String) -> (DynamicImage, ImageFormat) {
    let fd = Reader::open(path).unwrap();
    let format = fd.format().unwrap();
    let image = fd.decode().unwrap();
    return (image, format);
}
