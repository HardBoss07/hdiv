use hdim_render::{View, render};
use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};

#[test]
fn test_render_snapshot() {
    // 1. Setup Data (2x2 Grid)
    let mut buffer = RgbaImage::new(2, 2);
    buffer.put_pixel(0, 0, Rgba([255, 0, 0, 255])); // Red
    buffer.put_pixel(1, 0, Rgba([0, 0, 255, 255])); // Blue
    buffer.put_pixel(0, 1, Rgba([0, 255, 0, 255])); // Green
    buffer.put_pixel(1, 1, Rgba([255, 255, 255, 255])); // White
    let image = DynamicImage::ImageRgba8(buffer);

    // 2. Define the View to replicate old `area_size: 1`
    let (image_width, image_height) = image.dimensions();
    let area_size = 1;
    let view = View {
        source_x: 0,
        source_y: 0,
        source_width: image_width,
        source_height: image_height,
        target_width: image_width / area_size,
        target_height: image_height / (area_size * 2),
    };

    // 3. Run System Under Test
    let output = render(&image, &view).expect("Rendering failed");

    // Debug prints
    println!("Visual Output:\n{}", output);
    println!("Escaped String: {:?}", output);

    // 4. Verify with Snapshot
    // insta will now capture the ANSI 256 codes (\x1b[48;5;...m)
    insta::assert_snapshot!(output);
}
