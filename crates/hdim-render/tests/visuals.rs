use hdim_render::Renderer;
use image::{DynamicImage, Rgba, RgbaImage};

#[test]
fn test_render_snapshot() {
    // 1. Setup Data (2x2 Grid)
    let mut buffer = RgbaImage::new(2, 2);
    buffer.put_pixel(0, 0, Rgba([255, 0, 0, 255])); // Red
    buffer.put_pixel(1, 0, Rgba([0, 0, 255, 255])); // Blue
    buffer.put_pixel(0, 1, Rgba([0, 255, 0, 255])); // Green
    buffer.put_pixel(1, 1, Rgba([255, 255, 255, 255])); // White
    let img = DynamicImage::ImageRgba8(buffer);

    let renderer = Renderer::new(1);

    // 3. Run System Under Test
    let output = renderer.render(&img).expect("Rendering failed");

    // Debug prints
    println!("Visual Output:\n{}", output);
    println!("Escaped String: {:?}", output);

    // 4. Verify with Snapshot
    // insta will now capture the ANSI 256 codes (\x1b[48;5;...m)
    insta::assert_snapshot!(output);
}
