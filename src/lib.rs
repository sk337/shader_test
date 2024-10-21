mod color;
mod pixel_buffer;
mod point;
pub use color::{Color, Color3};
use pixel_buffer::PixelBuffer;
use png::ColorType;
pub use point::Point;
use std::fs::File;

/// A light source.
pub struct Light {
    pub x: f64,
    pub y: f64,
    pub color: Color,
    pub intensity: f64,
    pub direction: f64,
    pub fov: f64,
}

pub struct LightMapper {
    width: u64,
    height: u64,
    scale: u64,
    pixel_buffer: PixelBuffer<Color>,
    walls_texture: PixelBuffer<Color>,
    lights: Vec<Light>,
    walls: Vec<Vec<bool>>,
    do_pixel_check: bool,
}

impl LightMapper {
    pub fn new(
        width: u64,
        height: u64,
        scale: u64,
        wall_texture_path: &str,
        do_pixel_check: bool,
    ) -> LightMapper {
        let reader = png::Decoder::new(File::open(wall_texture_path).unwrap());
        let mut reader = reader.read_info().unwrap();
        let mut texture = vec![0; reader.output_buffer_size()];
        reader.next_frame(&mut texture).unwrap();

        let info = reader.info();

        assert!(
            info.color_type == ColorType::Rgba && info.width == 64 && info.height == 48,
            "Texture must be RGBA and 64x48 pixels in size"
        );

        let walls_texture = pixel_buffer::PixelBuffer::<Color>::from_buffer(texture, 48, 64);

        LightMapper {
            width,
            height,
            scale,
            pixel_buffer: pixel_buffer::PixelBuffer::<Color>::new(
                width * 8 * scale,
                height * 8 * scale,
            ),
            walls_texture,
            do_pixel_check,
            walls: vec![vec![false; width as usize]; height as usize],
        }
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn render(&mut self) {}

    fn render_walls(&self) -> PixelBuffer<Color> {
        let mut walls = PixelBuffer::<Color>::new(self.width * 8, self.height * 8);

        for y in 0..self.width * 8 {
            for x in 0..self.height * 8 {
                let mut color = Color {
                    r: 0x00,
                    g: 0x00,
                    b: 0x00,
                    a: 0xff,
                };

                walls[(y * (self.width * 8) + x) as usize] = color;
            }
        }

        walls
    }

    #[inline]
    fn is_within_square(&self, point: &Point) -> bool {
        let grid_x = (point.x) as usize;
        let grid_y = (point.y) as usize;
        if grid_x < self.width as usize && grid_y < self.height as usize {
            self.walls[grid_y][grid_x]
        } else {
            false
        }
    }

    fn get_surrounding_square_bitmap(&self, point: &Point) -> u8 {
        let mut bitmap: u8 = 0;

        let grid_x = (point.x) as usize;
        let grid_y = (point.y) as usize;

        if grid_x > 0 && grid_y > 0 && self.walls[grid_y - 1][grid_x - 1] {
            bitmap |= 0b1000_0000;
        }

        if grid_y > 0 && self.walls[grid_y - 1][grid_x] {
            bitmap |= 0b0100_0000;
        }

        if grid_x < self.width as usize - 1 && grid_y > 0 && self.walls[grid_y - 1][grid_x + 1] {
            bitmap |= 0b0010_0000;
        }

        if grid_x > 0 && self.walls[grid_y][grid_x - 1] {
            bitmap |= 0b0001_0000;
        }

        if grid_x < self.width as usize - 1 && self.walls[grid_y][grid_x + 1] {
            bitmap |= 0b0000_1000;
        }

        if grid_x > 0 && grid_y < self.height as usize - 1 && self.walls[grid_y + 1][grid_x - 1] {
            bitmap |= 0b0000_0100;
        }

        if grid_y < self.height as usize - 1 && self.walls[grid_y + 1][grid_x] {
            bitmap |= 0b0000_0010;
        }

        if grid_x < self.width as usize - 1
            && grid_y < self.height as usize - 1
            && self.walls[grid_y + 1][grid_x + 1]
        {
            bitmap |= 0b0000_0001;
        }

        bitmap
    }
}
