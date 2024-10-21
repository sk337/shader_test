use crate::{Color, Color3};
use std::ops::Index;

pub trait ColorTrait {}

impl ColorTrait for Color3 {}
impl ColorTrait for Color {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorSpace {
    RGB,
    RGBA,
}

pub struct PixelBuffer<T: ColorTrait> {
    color_space: ColorSpace,
    width: u64,
    height: u64,
    buffer: Vec<u8>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: ColorTrait> PixelBuffer<T> {
    pub fn new(color_space: ColorSpace, width: u64, height: u64) -> PixelBuffer<T> {
        if color_space == ColorSpace::RGB {
            let buffer = vec![0; (width * height * 3) as usize];
            PixelBuffer {
                color_space,
                width,
                height,
                buffer,
                _phantom: std::marker::PhantomData,
            }
        } else {
            let buffer = vec![0; (width * height * 4) as usize];
            PixelBuffer {
                color_space,
                width,
                height,
                buffer,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    pub fn buffer(&self) -> &Vec<u8> {
        &self.buffer
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

impl Index<usize> for PixelBuffer<Color> {
    type Output = Color;

    fn index(&self, index: usize) -> &Color {
        let start = index * 4;
        let end = start + 4;

        // Ensure the slice is the correct length
        let slice = &self.buffer[start..end];

        // Assuming Color is a struct with 4 components (RGBA), create the Color from the slice.
        let color = Color::from_rgba((slice[0], slice[1], slice[2], slice[3]));

        // Use Box to turn the Color into a reference, which should be a valid memory reference.
        Box::leak(Box::new(color))
    }
}

impl Index<usize> for PixelBuffer<Color3> {
    type Output = Color3;

    fn index(&self, index: usize) -> &Color3 {
        let start = index * 3;
        let end = start + 3;

        // Ensure the slice is the correct length
        let slice = &self.buffer[start..end];

        // Assuming Color3 is a struct with 3 components (RGB), create the Color3 from the slice.
        let color = Color3 {
            r: slice[0],
            g: slice[1],
            b: slice[2],
        };

        // Use Box to turn the Color3 into a reference, which should be a valid memory reference.
        Box::leak(Box::new(color))
    }
}
