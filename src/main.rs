use opencv::{
    core::{self, Scalar, Point, Rect},
    imgcodecs::{self, IMREAD_COLOR},
    imgproc,
    prelude::*,
    Result,
};

fn draw_rectangle_on_image(
    image_path: &str,
    output_path: &str,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> Result<()> {
    // Read the image
    let mut img = imgcodecs::imread(image_path, IMREAD_COLOR)?;

    // Define the rectangle
    let rect = Rect::new(x, y, width, height);

    // Define the color (BGR) and thickness
    let color = Scalar::new(0.0, 255.0, 0.0, 0.0); // Green color
    let thickness = 2;

    // Draw the rectangle
    imgproc::rectangle(&mut img, rect, color, thickness, imgproc::LINE_8, 0)?;

    // Save the image with the rectangle
    imgcodecs::imwrite(output_path, &img, &core::Vector::new())?;

    Ok(())
}

fn main() {
    let image_path = "test.jpg";
    let output_path = "output.jpg";
    let x = 50;
    let y = 50;
    let width = 100;
    let height = 200;

    match draw_rectangle_on_image(image_path, output_path, x, y, width, height) {
        Ok(_) => println!("Rectangle drawn and image saved to {}", output_path),
        Err(e) => eprintln!("Error: {}", e),
    }
}
