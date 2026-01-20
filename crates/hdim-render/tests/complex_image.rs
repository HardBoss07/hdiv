use hdim_render::{View, render};
use image::{GenericImageView, Rgba, RgbaImage};

#[test]
fn test_complex_render() {
    let raw_image = generate_benchmark_image();
    let image = image::DynamicImage::ImageRgba8(raw_image);

    // Define the View to replicate old `area_size: 2`
    let (image_width, image_height) = image.dimensions();
    let area_size = 2;
    let view = View {
        source_x: 0,
        source_y: 0,
        source_width: image_width,
        source_height: image_height,
        target_width: (image_width as f32 / area_size as f32).ceil() as u32,
        target_height: (image_height as f32 / (area_size as f32 * 2.0)).ceil() as u32,
    };

    // Run System Under Test
    let output = render(&image, &view).unwrap();

    // Debug statement
    println!("{}", output);

    // Verify with snapshot
    insta::assert_snapshot!(output);
}

fn generate_benchmark_image() -> RgbaImage {
    let size = 256;
    let mut image = RgbaImage::new(size, size);

    for y in 0..size {
        for x in 0..size {
            // 1. Background Gradient (Blue to Black)
            let b = (y as f32 / size as f32 * 255.0) as u8;
            let mut color = [0, 0, b, 255];

            // 2. Horizontal & Vertical White Lines (Grid test)
            if x % 32 == 0 || y % 32 == 0 {
                color = [200, 200, 200, 255];
            }

            // 3. Diagonal Red Line (Check aliasing/stepping)
            if x == y || x == (size - y - 1) {
                color = [255, 0, 0, 255];
            }

            // 4. Color Circles (Center check)
            let dx = x as i32 - (size / 2) as i32;
            let dy = y as i32 - (size / 2) as i32;
            let dist_sq = dx * dx + dy * dy;

            if dist_sq < 2500 {
                // Large Green Circle
                color = [0, 255, 0, 255];
            }
            if dist_sq < 400 {
                // Small Yellow Core
                color = [255, 255, 0, 255];
            }

            // 5. Brightness Ramp (Top edge)
            if y < 10 {
                let brightness = (x as f32 / size as f32 * 255.0) as u8;
                color = [brightness, brightness, brightness, 255];
            }

            image.put_pixel(x, y, Rgba(color));
        }
    }
    image
}
