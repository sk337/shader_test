use crate::{Color, Color3};
use std::ops::{Index, IndexMut};

pub trait ColorTrait {}

impl ColorTrait for Color3 {}
impl ColorTrait for Color {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorSpace {
    RGB,
    RGBA,
}

pub struct PixelBuffer<T: ColorTrait> {
    width: u64,
    height: u64,
    buffer: Vec<u8>,
    index: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: ColorTrait> PixelBuffer<T> {
    pub fn buffer(&self) -> &Vec<u8> {
        &self.buffer
    }

    pub fn clear_iter(&mut self) {
        self.index = 0;
    }

    pub fn buffer_mut(&mut self) -> &mut Vec<u8> {
        &mut self.buffer
    }

    pub fn width(&self) -> u64 {
        self.width
    }

    pub fn height(&self) -> u64 {
        self.height
    }
}

impl Iterator for PixelBuffer<Color> {
    type Item = Color;

    fn next(&mut self) -> Option<Self::Item> {
        let width = self.width as usize;
        let height = self.height as usize;
        if self.index < width * height {
            let color = Color::from_rgba((
                self.buffer[self.index],
                self.buffer[self.index + 1],
                self.buffer[self.index + 2],
                self.buffer[self.index + 3],
            ));
            self.index += 4;
            Some(color)
        } else {
            None
        }
    }
}

impl Iterator for PixelBuffer<Color3> {
    type Item = Color3;

    fn next(&mut self) -> Option<Self::Item> {
        let width = self.width as usize;
        let height = self.height as usize;
        if self.index < width * height {
            let color = Color3::from_rgb((
                self.buffer[self.index],
                self.buffer[self.index + 1],
                self.buffer[self.index + 2],
            ));
            self.index += 3;
            Some(color)
        } else {
            None
        }
    }
}

impl PixelBuffer<Color> {
    pub fn new(width: u64, height: u64) -> PixelBuffer<Color> {
        assert!(
            width > 0 && height > 0,
            "Width and height must be greater than 0"
        );
        let mut buffer = vec![0; (width * height * 4) as usize];
        let mut i = 0;
        for _ in 0..width {
            for _ in 0..height {
                buffer[i + 3] = 255;
                i += 4;
            }
        }
        PixelBuffer {
            index: 0,
            width,
            height,
            buffer,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn from_buffer(buffer: Vec<u8>, height: u64, width: u64) -> PixelBuffer<Color> {
        assert!(
            width > 0 && height > 0,
            "Width and height must be greater than 0"
        );
        assert!(
            buffer.len() == (width * height * 4) as usize,
            "Buffer length must be equal to width * height * 4"
        );
        PixelBuffer {
            width,
            height,
            buffer,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn upscale(&mut self, factor: u64) {
        let mut new_buffer = Vec::new();
        for i in 0..self.height {
            for _ in 0..factor {
                for j in 0..self.width {
                    let start = (i * self.width + j) as usize * 4;
                    for _ in 0..factor {
                        new_buffer.push(self.buffer[start]);
                        new_buffer.push(self.buffer[start + 1]);
                        new_buffer.push(self.buffer[start + 2]);
                        new_buffer.push(self.buffer[start + 3]);
                    }
                }
            }
        }
        self.buffer = new_buffer;
        self.width *= factor;
        self.height *= factor;
    }
}

impl PixelBuffer<Color3> {
    pub fn new(width: u64, height: u64) -> PixelBuffer<Color3> {
        assert!(
            width > 0 && height > 0,
            "Width and height must be greater than 0"
        );
        PixelBuffer {
            index: 0,
            width,
            height,
            buffer: vec![0; (width * height * 3) as usize],
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn from_buffer(buffer: Vec<u8>, height: u64, width: u64) -> PixelBuffer<Color3> {
        assert!(
            width > 0 && height > 0,
            "Width and height must be greater than 0"
        );
        assert!(
            buffer.len() == (width * height * 3) as usize,
            "Buffer length must be equal to width * height * 3"
        );
        PixelBuffer {
            width,
            height,
            buffer,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn upscale(&mut self, factor: u64) {
        let mut new_buffer = Vec::new();
        for i in 0..self.height {
            for _ in 0..factor {
                for j in 0..self.width {
                    let start = (i * self.width + j) as usize * 3;
                    for _ in 0..factor {
                        new_buffer.push(self.buffer[start]);
                        new_buffer.push(self.buffer[start + 1]);
                        new_buffer.push(self.buffer[start + 2]);
                    }
                }
            }
        }
        self.buffer = new_buffer;
        self.width *= factor;
        self.height *= factor;
    }

    pub fn merge(&mut self, other: PixelBuffer<Color>) {
        assert_eq!(self.width, other.width);
        assert_eq!(self.height, other.height);
        let mut i = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let other_pixel = other[i];
                let factor = other_pixel.a as f64 / 255.;
                let other_pixel = Color3 {
                    r: other_pixel.r,
                    g: other_pixel.g,
                    b: other_pixel.b,
                };
                let self_pixel = self[i];
                self[i] = other_pixel.blend(self_pixel, factor);
            }
        }
    }
}

impl Index<usize> for PixelBuffer<Color> {
    type Output = Color;

    fn index(&self, index: usize) -> &Color {
        let start = index * 4;
        let end = start + 4;
        let slice = &self.buffer[start..end];

        // Convert the slice into a reference to Color.
        unsafe { &*(slice.as_ptr() as *const Color) }
    }
}

impl IndexMut<usize> for PixelBuffer<Color> {
    fn index_mut(&mut self, index: usize) -> &mut Color {
        let start = index * 4;
        let slice = &mut self.buffer[start..start + 4];

        // Convert the slice into a mutable reference to Color.
        unsafe { &mut *(slice.as_mut_ptr() as *mut Color) }
    }
}

impl Index<usize> for PixelBuffer<Color3> {
    type Output = Color3;

    fn index(&self, index: usize) -> &Color3 {
        let start = index * 3;
        let end = start + 3;
        let slice = &self.buffer[start..end];

        // Convert the slice into a reference to Color3.
        unsafe { &*(slice.as_ptr() as *const Color3) }
    }
}

impl IndexMut<usize> for PixelBuffer<Color3> {
    fn index_mut(&mut self, index: usize) -> &mut Color3 {
        let start = index * 4;
        let slice = &mut self.buffer[start..start + 3];

        // Convert the slice into a mutable reference to Color.
        unsafe { &mut *(slice.as_mut_ptr() as *mut Color3) }
    }
}
