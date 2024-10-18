use std::fs::File;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    fn blend(&self, other: Color, factor: f64) -> Color {
        let factor = factor.clamp(0.0, 1.0);
        Color {
            r: (self.r as f64 * factor + other.r as f64 * (1.0 - factor)) as u8,
            g: (self.g as f64 * factor + other.g as f64 * (1.0 - factor)) as u8,
            b: (self.b as f64 * factor + other.b as f64 * (1.0 - factor)) as u8,
            a: (self.a as f64 * factor + other.a as f64 * (1.0 - factor)) as u8,
        }
    }
}

#[derive(Debug)]
pub struct Light {
    pub position: Point,
    pub color: Color,
    pub intensity: f64, // The maximum distance the light reaches
}

#[derive(Debug)]
pub struct Map {
    pub height: u64,
    pub width: u64,
    pub sim_scale: u64,
    pub lights: Vec<Light>,
    pub squares: Vec<Vec<bool>>,
    pub pixel_buffer: Vec<u8>,
    pub texture: Vec<u8>,
}

impl Map {
    pub fn new(height: u64, width: u64, sim_scale: u64, texure_path: String) -> Map {
        let reader = png::Decoder::new(File::open(texure_path).unwrap());
        let mut reader = reader.read_info().unwrap();
        let mut texture = vec![0; reader.output_buffer_size()];
        reader.next_frame(&mut texture).unwrap();
        Map {
            height,
            width,
            sim_scale,
            lights: Vec::new(),
            squares: vec![vec![false; width as usize]; height as usize],
            pixel_buffer: vec![
                0;
                ((height * 8 * sim_scale) * (width * 8 * sim_scale) * 3) as usize
            ],
            texture,
        }
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
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

    pub fn color_walls(&mut self) {
        let mut i = 0;
        for y in 0..self.height * 8 * self.sim_scale {
            for x in 0..self.width * 8 * self.sim_scale {
                let scaled_point = Point {
                    x: x as f64 / 8. / self.sim_scale as f64,
                    y: y as f64 / 8. / self.sim_scale as f64,
                };
                if self.is_within_square(&scaled_point) {
                    let bitmask = self.get_surrounding_square_bitmap(&scaled_point);

                    let (tex_x, tex_y) = self.get_tex_cord(&scaled_point, bitmask);

                    let color = Color {
                        r: self.texture[(tex_y * 64 + tex_x) as usize * 4],
                        g: self.texture[(tex_y * 64 + tex_x) as usize * 4 + 1],
                        b: self.texture[(tex_y * 64 + tex_x) as usize * 4 + 2],
                        a: 0xff,
                    };
                    self.pixel_buffer[i] = color.r;
                    self.pixel_buffer[i + 1] = color.g;
                    self.pixel_buffer[i + 2] = color.b;
                }
                i += 3;
            }
        }
    }

    pub fn render(&mut self) {
        self.color_walls();
        let mut i = 0;
        for y in 0..self.height * 8 * self.sim_scale {
            for x in 0..self.width * 8 * self.sim_scale {
                let scaled_point = Point {
                    x: x as f64 / 8. / self.sim_scale as f64,
                    y: y as f64 / 8. / self.sim_scale as f64,
                };

                let mut pixel_color = Color {
                    r: 0x00,
                    g: 0x00,
                    b: 0x00,
                    a: 0xff,
                };

                if !self.is_within_square(&scaled_point) {
                    for light in &self.lights {
                        let distance = ((light.position.x - scaled_point.x).powi(2)
                            + (light.position.y - scaled_point.y).powi(2))
                        .sqrt();

                        if distance < light.intensity
                            && self.point_has_los(&light.position, &scaled_point)
                        {
                            let factor = 1.0 - distance / light.intensity;
                            // println!("{}", factor);
                            pixel_color = light.color.blend(pixel_color, factor);
                        }
                    }

                    self.pixel_buffer[i] = pixel_color.r;
                    self.pixel_buffer[i + 1] = pixel_color.g;
                    self.pixel_buffer[i + 2] = pixel_color.b;
                }

                i += 3;
            }
        }
    }

    fn is_within_square(&self, point: &Point) -> bool {
        let grid_x = (point.x) as usize;
        let grid_y = (point.y) as usize;
        if grid_x < self.width as usize && grid_y < self.height as usize {
            self.squares[grid_y][grid_x]
        } else {
            false
        }
    }

    fn get_root_square(&self, point: &Point) -> Point {
        Point {
            x: ((point.x * 8.) % 8.).floor(),
            y: ((point.y * 8.) % 8.).floor(),
        }
    }

    fn point_has_los(&self, a: &Point, b: &Point) -> bool {
        let dx = b.x - a.x;
        let dy = b.y - a.y;
        let distance = (dx.powi(2) + dy.powi(2)).sqrt();

        let steps = distance.ceil() as usize * 20;
        let dx = dx / steps as f64;
        let dy = dy / steps as f64;

        for i in 0..steps {
            let x = a.x + dx * i as f64;
            let y = a.y + dy * i as f64;

            if self.is_within_square(&Point { x, y }) {
                return false;
            }
        }

        true
    }

    fn get_surrounding_square_bitmap(&self, point: &Point) -> u8 {
        let mut bitmap: u8 = 0;

        let grid_x = (point.x) as usize;
        let grid_y = (point.y) as usize;

        if grid_x > 0 && grid_y > 0 && self.squares[grid_y - 1][grid_x - 1] {
            bitmap |= 0b1000_0000;
        }

        if grid_y > 0 && self.squares[grid_y - 1][grid_x] {
            bitmap |= 0b0100_0000;
        }

        if grid_x < self.width as usize - 1 && grid_y > 0 && self.squares[grid_y - 1][grid_x + 1] {
            bitmap |= 0b0010_0000;
        }

        if grid_x > 0 && self.squares[grid_y][grid_x - 1] {
            bitmap |= 0b0001_0000;
        }

        if grid_x < self.width as usize - 1 && self.squares[grid_y][grid_x + 1] {
            bitmap |= 0b0000_1000;
        }

        if grid_x > 0 && grid_y < self.height as usize - 1 && self.squares[grid_y + 1][grid_x - 1] {
            bitmap |= 0b0000_0100;
        }

        if grid_y < self.height as usize - 1 && self.squares[grid_y + 1][grid_x] {
            bitmap |= 0b0000_0010;
        }

        if grid_x < self.width as usize - 1
            && grid_y < self.height as usize - 1
            && self.squares[grid_y + 1][grid_x + 1]
        {
            bitmap |= 0b0000_0001;
        }

        bitmap
    }

    fn get_tex_cord(&self, point: &Point, bitmap: u8) -> (u32, u32) {
        let x: u32;
        let y: u32;
        let root_square = self.get_root_square(point);

        // println!("{:?}", root_square);

        match bitmap {
            0b000_00_000 => {
                x = 56;
                y = 0;
            }
            0b100_00_000 => {
                x = 56;
                y = 0;
            }
            0b010_00_000 => {
                x = 48;
                y = 16;
            }
            0b001_00_000 => {
                x = 56;
                y = 0;
            }
            0b000_10_000 => {
                x = 16;
                y = 24;
            }
            0b000_01_000 => {
                x = 0;
                y = 24;
            }
            0b000_00_100 => {
                x = 56;
                y = 0;
            }
            0b000_00_010 => {
                x = 48;
                y = 0;
            }
            0b000_00_001 => {
                x = 56;
                y = 0;
            }
            0b000_00_011 => {
                x = 48;
                y = 0;
            }
            0b000_00_101 => {
                x = 56;
                y = 0;
            }
            0b000_00_110 => {
                x = 48;
                y = 0;
            }
            0b000_00_111 => {
                x = 48;
                y = 0;
            }
            0b000_01_001 => {
                x = 0;
                y = 24;
            }
            0b000_01_010 => {
                x = 56;
                y = 16;
            }
            0b000_01_011 => {
                x = 0;
                y = 0;
            }
            0b000_01_100 => {
                x = 0;
                y = 24;
            }
            0b000_01_101 => {
                x = 0;
                y = 24;
            }
            0b000_01_110 => {
                x = 56;
                y = 16;
            }
            0b000_01_111 => {
                x = 0;
                y = 0;
            }
            0b000_10_001 => {
                x = 16;
                y = 24;
            }
            0b000_10_010 => {
                x = 56;
                y = 8;
            }
            0b000_10_011 => {
                x = 56;
                y = 8;
            }
            0b000_10_100 => {
                x = 16;
                y = 24;
            }
            0b000_10_101 => {
                x = 16;
                y = 24;
            }
            0b000_10_110 => {
                x = 16;
                y = 0;
            }
            0b000_10_111 => {
                x = 16;
                y = 0;
            }
            0b000_11_000 => {
                x = 8;
                y = 24;
            }
            0b000_11_001 => {
                x = 8;
                y = 24;
            }

            0b000_11_010 => {
                x = 8;
                y = 40;
            }

            0b000_11_011 => {
                x = 16;
                y = 40;
            }

            0b000_11_100 => {
                x = 8;
                y = 24;
            }

            0b000_11_101 => {
                x = 8;
                y = 24;
            }

            0b000_11_110 => {
                x = 0;
                y = 40;
            }

            0b000_11_111 => {
                x = 8;
                y = 0;
            }
            0b001_00_001 => {
                x = 56;
                y = 0;
            }
            0b001_00_010 => {
                x = 48;
                y = 0;
            }
            0b001_00_011 => {
                x = 48;
                y = 0;
            }
            0b001_00_100 => {
                x = 56;
                y = 0;
            }
            0b001_00_101 => {
                x = 56;
                y = 0;
            }
            0b001_00_110 => {
                x = 48;
                y = 0;
            }
            0b001_00_111 => {
                x = 48;
                y = 0;
            }
            0b001_01_000 => {
                x = 0;
                y = 24;
            }
            0b001_01_001 => {
                x = 0;
                y = 24;
            }
            0b001_01_010 => {
                x = 56;
                y = 16;
            }
            0b001_01_011 => {
                x = 0;
                y = 0;
            }
            0b001_01_100 => {
                x = 0;
                y = 24;
            }
            0b001_01_101 => {
                x = 0;
                y = 24;
            }
            0b001_01_110 => {
                x = 56;
                y = 16;
            }
            0b001_01_111 => {
                x = 0;
                y = 0;
            }
            0b001_10_000 => {
                x = 16;
                y = 24;
            }
            0b001_10_001 => {
                x = 16;
                y = 24;
            }
            0b001_10_010 => {
                x = 56;
                y = 8;
            }
            0b001_10_011 => {
                x = 56;
                y = 8;
            }
            0b001_10_100 => {
                x = 16;
                y = 24;
            }
            0b001_10_101 => {
                x = 16;
                y = 24;
            }
            0b001_10_110 => {
                x = 16;
                y = 0;
            }
            0b001_10_111 => {
                x = 16;
                y = 0;
            }
            0b001_11_000 => {
                x = 8;
                y = 24;
            }
            0b001_11_001 => {
                x = 8;
                y = 24;
            }
            0b001_11_010 => {
                x = 8;
                y = 40;
            }
            0b001_11_011 => {
                x = 16;
                y = 40;
            }
            0b001_11_100 => {
                x = 8;
                y = 24;
            }
            0b001_11_101 => {
                x = 8;
                y = 24;
            }
            0b001_11_110 => {
                x = 0;
                y = 40;
            }
            0b001_11_111 => {
                x = 8;
                y = 0;
            }
            0b010_00_001 => {
                x = 48;
                y = 16;
            }
            0b010_00_010 => {
                x = 48;
                y = 8;
            }
            0b010_00_011 => {
                x = 48;
                y = 8;
            }
            0b010_00_100 => {
                x = 48;
                y = 16;
            }
            0b010_00_101 => {
                x = 48;
                y = 16;
            }
            0b010_00_110 => {
                x = 48;
                y = 8;
            }
            0b010_00_111 => {
                x = 48;
                y = 8;
            }
            0b010_01_000 => {
                x = 56;
                y = 24;
            }
            0b010_01_001 => {
                x = 56;
                y = 24;
            }
            0b010_01_010 => {
                x = 48;
                y = 32;
            }
            0b010_01_011 => {
                x = 48;
                y = 40;
            }
            0b010_01_100 => {
                x = 56;
                y = 24;
            }
            0b010_01_101 => {
                x = 56;
                y = 24;
            }
            0b010_01_110 => {
                x = 48;
                y = 32;
            }
            0b010_01_111 => {
                x = 48;
                y = 40;
            }
            0b010_10_000 => {
                x = 56;
                y = 32;
            }
            0b010_10_001 => {
                x = 56;
                y = 32;
            }
            0b010_10_010 => {
                x = 40;
                y = 32;
            }
            0b010_10_011 => {
                x = 40;
                y = 32;
            }
            0b010_10_100 => {
                x = 56;
                y = 32;
            }
            0b010_10_101 => {
                x = 56;
                y = 32;
            }
            0b010_10_110 => {
                x = 40;
                y = 40;
            }
            0b010_10_111 => {
                x = 40;
                y = 40;
            }
            0b010_11_000 => {
                x = 8;
                y = 32;
            }
            0b010_11_001 => {
                x = 8;
                y = 32;
            }
            0b010_11_010 => {
                x = 32;
                y = 8;
            }
            0b010_11_011 => {
                x = 32;
                y = 32;
            }
            0b010_11_100 => {
                x = 8;
                y = 32;
            }
            0b010_11_101 => {
                x = 8;
                y = 32;
            }
            0b010_11_110 => {
                x = 24;
                y = 32;
            }
            0b010_11_111 => {
                x = 32;
                y = 16;
            }
            0b011_00_000 => {
                x = 48;
                y = 16;
            }
            0b011_00_001 => {
                x = 48;
                y = 16;
            }
            0b011_00_010 => {
                x = 48;
                y = 8;
            }
            0b011_00_011 => {
                x = 48;
                y = 8;
            }
            0b011_00_100 => {
                x = 48;
                y = 16;
            }
            0b011_00_101 => {
                x = 48;
                y = 16;
            }
            0b011_00_110 => {
                x = 48;
                y = 8;
            }
            0b011_00_111 => {
                x = 48;
                y = 8;
            }
            0b011_01_000 => {
                x = 0;
                y = 16;
            }
            0b011_01_001 => {
                x = 0;
                y = 16;
            }
            0b011_01_010 => {
                x = 48;
                y = 24;
            }
            0b011_01_011 => {
                x = 0;
                y = 8;
            }
            0b011_01_100 => {
                x = 0;
                y = 16;
            }
            0b011_01_101 => {
                x = 0;
                y = 16;
            }
            0b011_01_110 => {
                x = 48;
                y = 24;
            }
            0b011_01_111 => {
                x = 0;
                y = 8;
            }
            0b011_10_000 => {
                x = 56;
                y = 32;
            }
            0b011_10_001 => {
                x = 56;
                y = 32;
            }
            0b011_10_010 => {
                x = 40;
                y = 32;
            }
            0b011_10_011 => {
                x = 40;
                y = 32;
            }
            0b011_10_100 => {
                x = 56;
                y = 32;
            }
            0b011_10_101 => {
                x = 56;
                y = 32;
            }
            0b011_10_110 => {
                x = 40;
                y = 40;
            }
            0b011_10_111 => {
                x = 40;
                y = 40;
            }
            0b011_11_000 => {
                x = 16;
                y = 32;
            }
            0b011_11_001 => {
                x = 16;
                y = 32;
            }
            0b011_11_010 => {
                x = 32;
                y = 24;
            }
            0b011_11_011 => {
                x = 40;
                y = 8;
            }
            0b011_11_100 => {
                x = 16;
                y = 32;
            }
            0b011_11_101 => {
                x = 16;
                y = 32;
            }
            0b011_11_110 => {
                x = 24;
                y = 40;
            }
            0b011_11_111 => {
                x = 40;
                y = 16;
            }
            0b100_00_001 => {
                x = 56;
                y = 0;
            }
            0b100_00_010 => {
                x = 48;
                y = 0;
            }
            0b100_00_011 => {
                x = 48;
                y = 0;
            }
            0b100_00_100 => {
                x = 8;
                y = 8;
            }
            0b100_00_101 => {
                x = 8;
                y = 8;
            }
            0b100_00_110 => {
                x = 48;
                y = 0;
            }
            0b100_00_111 => {
                x = 48;
                y = 0;
            }
            0b100_01_000 => {
                x = 0;
                y = 24;
            }
            0b100_01_001 => {
                x = 0;
                y = 24;
            }
            0b100_01_010 => {
                x = 56;
                y = 16;
            }
            0b100_01_011 => {
                x = 0;
                y = 0;
            }
            0b100_01_100 => {
                x = 0;
                y = 24;
            }
            0b100_01_101 => {
                x = 0;
                y = 24;
            }
            0b100_01_110 => {
                x = 56;
                y = 16;
            }
            0b100_01_111 => {
                x = 0;
                y = 0;
            }
            0b100_10_000 => {
                x = 16;
                y = 24;
            }
            0b100_10_001 => {
                x = 16;
                y = 24;
            }
            0b100_10_010 => {
                x = 56;
                y = 8;
            }
            0b100_10_011 => {
                x = 56;
                y = 8;
            }
            0b100_10_100 => {
                x = 16;
                y = 24;
            }
            0b100_10_101 => {
                x = 16;
                y = 24;
            }
            0b100_10_110 => {
                x = 16;
                y = 0;
            }
            0b100_10_111 => {
                x = 16;
                y = 0;
            }
            0b100_11_000 => {
                x = 8;
                y = 24;
            }
            0b100_11_001 => {
                x = 8;
                y = 24;
            }
            0b100_11_010 => {
                x = 8;
                y = 40;
            }
            0b100_11_011 => {
                x = 16;
                y = 40;
            }
            0b100_11_100 => {
                x = 8;
                y = 24;
            }
            0b100_11_101 => {
                x = 8;
                y = 24;
            }
            0b100_11_110 => {
                x = 0;
                y = 40;
            }
            0b100_11_111 => {
                x = 8;
                y = 0;
            }
            0b101_00_000 => {
                x = 8;
                y = 8;
            }
            0b101_00_001 => {
                x = 8;
                y = 8;
            }
            0b101_00_010 => {
                x = 48;
                y = 0;
            }
            0b101_00_011 => {
                x = 48;
                y = 0;
            }
            0b101_00_100 => {
                x = 8;
                y = 8;
            }
            0b101_00_101 => {
                x = 8;
                y = 8;
            }
            0b101_00_110 => {
                x = 48;
                y = 0;
            }
            0b101_00_111 => {
                x = 48;
                y = 0;
            }
            0b101_01_000 => {
                x = 0;
                y = 24;
            }
            0b101_01_001 => {
                x = 0;
                y = 24;
            }
            0b101_01_010 => {
                x = 56;
                y = 16;
            }
            0b101_01_011 => {
                x = 0;
                y = 0;
            }
            0b101_01_100 => {
                x = 0;
                y = 24;
            }
            0b101_01_101 => {
                x = 0;
                y = 24;
            }
            0b101_01_110 => {
                x = 56;
                y = 16;
            }
            0b101_01_111 => {
                x = 0;
                y = 0;
            }
            0b101_10_000 => {
                x = 16;
                y = 24;
            }
            0b101_10_001 => {
                x = 16;
                y = 24;
            }
            0b101_10_010 => {
                x = 56;
                y = 8;
            }
            0b101_10_011 => {
                x = 56;
                y = 8;
            }
            0b101_10_100 => {
                x = 16;
                y = 24;
            }
            0b101_10_101 => {
                x = 16;
                y = 24;
            }
            0b101_10_110 => {
                x = 16;
                y = 0;
            }
            0b101_10_111 => {
                x = 16;
                y = 0;
            }
            0b101_11_000 => {
                x = 8;
                y = 24;
            }
            0b101_11_001 => {
                x = 8;
                y = 24;
            }
            0b101_11_010 => {
                x = 8;
                y = 40;
            }
            0b101_11_011 => {
                x = 16;
                y = 40;
            }
            0b101_11_100 => {
                x = 8;
                y = 24;
            }
            0b101_11_101 => {
                x = 8;
                y = 24;
            }
            0b101_11_110 => {
                x = 0;
                y = 40;
            }
            0b101_11_111 => {
                x = 8;
                y = 0;
            }
            0b110_00_000 => {
                x = 48;
                y = 16;
            }
            0b110_00_001 => {
                x = 48;
                y = 16;
            }
            0b110_00_010 => {
                x = 48;
                y = 8;
            }
            0b110_00_011 => {
                x = 48;
                y = 8;
            }
            0b110_00_100 => {
                x = 48;
                y = 16;
            }
            0b110_00_101 => {
                x = 48;
                y = 16;
            }
            0b110_00_110 => {
                x = 48;
                y = 8;
            }
            0b110_00_111 => {
                x = 48;
                y = 8;
            }
            0b110_01_000 => {
                x = 56;
                y = 24;
            }
            0b110_01_001 => {
                x = 56;
                y = 24;
            }
            0b110_01_010 => {
                x = 48;
                y = 32;
            }
            0b110_01_011 => {
                x = 48;
                y = 40;
            }
            0b110_01_100 => {
                x = 56;
                y = 24;
            }
            0b110_01_101 => {
                x = 56;
                y = 24;
            }
            0b110_01_110 => {
                x = 48;
                y = 32;
            }
            0b110_01_111 => {
                x = 48;
                y = 40;
            }
            0b110_10_000 => {
                x = 16;
                y = 16;
            }
            0b110_10_001 => {
                x = 16;
                y = 16;
            }
            0b110_10_010 => {
                x = 40;
                y = 24;
            }
            0b110_10_011 => {
                x = 40;
                y = 24;
            }
            0b110_10_100 => {
                x = 16;
                y = 16;
            }
            0b110_10_101 => {
                x = 16;
                y = 16;
            }
            0b110_10_110 => {
                x = 16;
                y = 8;
            }
            0b110_10_111 => {
                x = 16;
                y = 8;
            }
            0b110_11_000 => {
                x = 0;
                y = 32;
            }
            0b110_11_001 => {
                x = 0;
                y = 32;
            }
            0b110_11_010 => {
                x = 24;
                y = 24;
            }
            0b110_11_011 => {
                x = 32;
                y = 40;
            }
            0b110_11_100 => {
                x = 0;
                y = 32;
            }
            0b110_11_101 => {
                x = 0;
                y = 32;
            }
            0b110_11_110 => {
                x = 24;
                y = 8;
            }
            0b110_11_111 => {
                x = 24;
                y = 16;
            }
            0b111_00_000 => {
                x = 48;
                y = 16;
            }
            0b111_00_001 => {
                x = 48;
                y = 16;
            }
            0b111_00_010 => {
                x = 48;
                y = 8;
            }
            0b111_00_011 => {
                x = 48;
                y = 8;
            }
            0b111_00_100 => {
                x = 48;
                y = 16;
            }
            0b111_00_101 => {
                x = 48;
                y = 16;
            }
            0b111_00_110 => {
                x = 48;
                y = 8;
            }
            0b111_00_111 => {
                x = 48;
                y = 8;
            }
            0b111_01_000 => {
                x = 0;
                y = 16;
            }
            0b111_01_001 => {
                x = 0;
                y = 16;
            }
            0b111_01_010 => {
                x = 48;
                y = 24;
            }
            0b111_01_011 => {
                x = 0;
                y = 8;
            }
            0b111_01_100 => {
                x = 0;
                y = 16;
            }
            0b111_01_101 => {
                x = 0;
                y = 16;
            }
            0b111_01_110 => {
                x = 48;
                y = 24;
            }
            0b111_01_111 => {
                x = 0;
                y = 8;
            }
            0b111_10_000 => {
                x = 16;
                y = 16;
            }
            0b111_10_001 => {
                x = 16;
                y = 16;
            }
            0b111_10_010 => {
                x = 40;
                y = 24;
            }
            0b111_10_011 => {
                x = 40;
                y = 24;
            }
            0b111_10_100 => {
                x = 16;
                y = 16;
            }
            0b111_10_101 => {
                x = 16;
                y = 16;
            }
            0b111_10_110 => {
                x = 16;
                y = 8;
            }
            0b111_10_111 => {
                x = 16;
                y = 8;
            }
            0b111_11_000 => {
                x = 8;
                y = 16;
            }
            0b111_11_001 => {
                x = 8;
                y = 16;
            }
            0b111_11_010 => {
                x = 32;
                y = 0;
            }
            0b111_11_011 => {
                x = 40;
                y = 0;
            }
            0b111_11_100 => {
                x = 8;
                y = 16;
            }
            0b111_11_101 => {
                x = 8;
                y = 16;
            }
            0b111_11_110 => {
                x = 24;
                y = 0;
            }
            0b111_11_111 => {
                x = 8;
                y = 8;
            }
        }

        // println!("{:?}", point);

        (x + root_square.x as u32, y + root_square.y as u32)
    }
}
