
use image::{DynamicImage, GenericImageView, Rgba, imageops::FilterType};
use ndarray::{Array4, ArrayBase};

fn image_to_yolo(original_image: &DynamicImage) -> Array4<f32> {
    let mut input = ArrayBase::zeros((1, 3, 640, 640));

    let image = original_image.resize_exact(640, 640, FilterType::CatmullRom);
    for (x, y, Rgba([r, g, b, _])) in image.pixels() {
        let x = x as usize;
        let y = y as usize;

        input[[0, 0, y, x]] = (r as f32) / 255.;
        input[[0, 1, y, x]] = (g as f32) / 255.;
        input[[0, 2, y, x]] = (b as f32) / 255.;
    }

    input
}

fn image_to_yolo_two(original_image: &DynamicImage) -> Array4<f32> {
    let mut input = ArrayBase::zeros((1, 640, 640, 3));

    let image = original_image.resize_exact(640, 640, FilterType::CatmullRom);
    for (x, y, Rgba([r, g, b, _])) in image.pixels() {
        let x = x as usize;
        let y = y as usize;

        input[[0, y, x, 0]] = (r as f32) / 255.;
        input[[0, y, x, 1]] = (g as f32) / 255.;
        input[[0, y, x, 2]] = (b as f32) / 255.;
    }

    input
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open("54c08a9-life-shura705.jpg")?;
    let yolo_im = image_to_yolo(&img);

    let yolo_reshaped = yolo_im.clone().into_shape_with_order((1, 640, 640, 3)).unwrap();

    println!("Image: {:?}", yolo_im.shape());

    println!("Image reshaped: {:?}", yolo_reshaped.shape());

    let yolo_im2 = image_to_yolo_two(&img);

    println!("Image 2: {:?}", yolo_im2.shape());

    // println!("{:#?}", yolo_im);
    // println!("{:#?}", yolo_im2);

    Ok(())
}
