use std::convert::TryInto;
use std::fs::File;

const STEPS: usize = 200;
const OCCUPIED: Color = Color {
    r: 255,
    g: 0,
    b: 255,
};

const WIDTH: u64 = 32;
const HEIGHT: u64 = 16;
const SCALE: u64 = 4;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn blend(&self, other: Color, factor: f64) -> Color {
        let factor = factor.clamp(0.0, 1.0);
        Color {
            r: (self.r as f64 * (1.0 - factor) + other.r as f64 * factor) as u8,
            g: (self.g as f64 * (1.0 - factor) + other.g as f64 * factor) as u8,
            b: (self.b as f64 * (1.0 - factor) + other.b as f64 * factor) as u8,
        }
    }
}

#[derive(Debug)]
struct Light {
    position: Point,
    color: Color,
    intensity: f64, // The maximum distance the light reaches
}

#[derive(Debug)]
struct Map {
    height: u64,
    width: u64,
    lights: Vec<Light>,
    squares: Vec<Vec<bool>>,
    pixel_buffer: Vec<u8>,
}

impl Map {
    pub fn new(height: u64, width: u64) -> Map {
        Map {
            height,
            width,
            lights: Vec::new(),
            squares: vec![vec![false; width as usize]; height as usize],
            pixel_buffer: vec![0; ((height * SCALE) * (width * SCALE) * 3) as usize],
        }
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn set_squares(&mut self, l: Vec<Vec<bool>>) {
        self.squares = l;
    }

    pub fn squares_from_file(&mut self, path: String) {
        let contents =
            std::fs::read_to_string(path).expect("Something went wrong reading the file");
        contents.lines().enumerate().for_each(|(i, line)| {
            line.chars().enumerate().for_each(|(j, c)| {
                if c == '#' {
                    self.squares[i][j] = true;
                }
            });
        });
    }

    fn has_line_of_sight(&self, start: Point, end: Point) -> bool {
        let mut current_position = start;
        let direction = Point {
            x: end.x - start.x,
            y: end.y - start.y,
        };
        let step_size = 1.0 / STEPS as f64;

        for _ in 0..STEPS {
            let x = current_position.x as usize;
            let y = current_position.y as usize;

            if x < self.width as usize && y < self.height as usize && self.squares[y][x] {
                return false; // Obstacle detected, no line of sight
            }

            current_position.x += direction.x * step_size;
            current_position.y += direction.y * step_size;
        }

        true // No obstacles detected, line of sight exists
    }

    pub fn render(&mut self) {
        for y in 0..(self.height * SCALE) {
            for x in 0..(self.width * SCALE) {
                let pixel_position = Point {
                    x: x as f64 / SCALE as f64,
                    y: y as f64 / SCALE as f64,
                };

                let mut pixel_color = Color { r: 0, g: 0, b: 0 };

                // Check if the pixel is inside a square
                let square_x = (x / SCALE) as usize;
                let square_y = (y / SCALE) as usize;
                if square_x < self.width as usize && square_y < self.height as usize {
                    if self.squares[square_y][square_x] {
                        // Color the square with OCCUPIED color
                        pixel_color = OCCUPIED;
                    } else {
                        // Calculate lighting for each light
                        for light in &self.lights {
                            if self.has_line_of_sight(pixel_position, light.position) {
                                let distance = ((light.position.x - pixel_position.x).powi(2)
                                    + (light.position.y - pixel_position.y).powi(2))
                                .sqrt();

                                if distance < light.intensity {
                                    // Calculate the light factor based on linear fading
                                    let light_factor = 1.0 - (distance / light.intensity);
                                    pixel_color = pixel_color.blend(light.color, light_factor);
                                }
                            }
                        }
                    }
                }

                // Set the pixel color in the buffer
                let index = ((y * (self.width * SCALE) + x) * 3) as usize;
                self.pixel_buffer[index] = pixel_color.r;
                self.pixel_buffer[index + 1] = pixel_color.g;
                self.pixel_buffer[index + 2] = pixel_color.b;
            }
        }
    }
}

fn main() {
    let mut map = Map::new(HEIGHT, WIDTH);
    map.add_light(Light {
        position: Point { x: 16.0, y: 8.0 },
        color: Color {
            r: 0x99,
            g: 0x99,
            b: 0x99,
        },
        intensity: 10.0, // Light fades out completely at a distance of 10 units
    });

    map.add_light(Light {
        position: Point { x: 3.5, y: 8.5 },
        color: Color {
            r: 0x99,
            g: 0x99,
            b: 0x99,
        },
        intensity: 10.0, // Light fades out completely at a distance of 10 units
    });

    map.squares_from_file("map.txt".to_string());

    // Render the scene with ray tracing
    map.render();

    // Save the pixel buffer to a PNG file
    let width: u32 = (WIDTH * SCALE).try_into().unwrap();
    let height: u32 = (HEIGHT * SCALE).try_into().unwrap();

    let mut encoder = png::Encoder::new(File::create("output.png").unwrap(), width, height);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&map.pixel_buffer).unwrap();
    writer.finish().unwrap();

    println!("Done!");
}
