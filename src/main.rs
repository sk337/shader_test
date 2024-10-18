use std::convert::TryInto;
use std::fs::File;

mod map;

use map::{Color, Light, Map, Point};

static WIDTH: u64 = 32;
static HEIGHT: u64 = 16;
static SCALE: u64 = 8;

fn main() {
    let mut map = Map::new(HEIGHT, WIDTH, SCALE, "texture-base.png".to_string());
    map.add_light(Light {
        position: Point { x: 16.0, y: 8.0 },
        color: Color {
            r: 0x99,
            g: 0x99,
            b: 0x99,
            a: 0xff,
        },
        intensity: 10.0, // Light fades out completely at a distance of 10 units
    });

    map.add_light(Light {
        position: Point { x: 4., y: 9. },
        color: Color {
            r: 0xff,
            g: 0xff,
            b: 0x00,
            a: 0xff,
        },
        intensity: 10.0, // Light fades out completely at a distance of 10 units
    });

    map.squares_from_file("map.txt".to_string());

    // Render the scene with ray tracing
    println!("Rendering...");
    map.render();

    // Save the pixel buffer to a PNG file
    let width: u32 = (WIDTH * SCALE * 8).try_into().unwrap();
    let height: u32 = (HEIGHT * SCALE * 8).try_into().unwrap();

    println!("Saving to output.png...");
    let mut encoder = png::Encoder::new(File::create("output.png").unwrap(), width, height);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&map.pixel_buffer).unwrap();
    writer.finish().unwrap();

    println!("Done!");
}
