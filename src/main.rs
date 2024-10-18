use shader_test::{Color, Light, Map, Point};

static WIDTH: u64 = 32;
static HEIGHT: u64 = 16;
static SCALE: u64 = 8;

fn main() {
    let mut map = Map::new(
        HEIGHT,
        WIDTH,
        SCALE,
        "texture-base.png".to_string(),
        0.1,
        1.0,
    );
    map.squares_from_file("map.txt".to_string());

    // add lights
    // map.add_light(Light {
    //     position: Point { x: 16.0, y: 8.0 },
    //     color: Color {
    //         r: 255,
    //         g: 255,
    //         b: 255,
    //         a: 255,
    //     },
    //     intensity: 10.0,
    //     angle: 0.0,
    //     fov: 90.0,
    // });

    // Render the scene with ray tracing
    println!("Rendering...");
    map.render();

    println!("Saving to output.png...");
    map.save_to_file("output2.png");

    println!("Done!");
}
