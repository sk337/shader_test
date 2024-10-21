#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /// Blend two colors based on a given factor.
    ///
    /// # Arguments
    ///
    /// * `other` - The color to blend with.
    /// * `factor` - The blend factor (0.0 to 1.0), where 0.0 returns `self` and 1.0 returns `other`.
    ///
    /// # Returns
    ///
    /// A new `Color` resulting from the blend.
    pub fn blend(&self, other: Color, factor: f64) -> Color {
        let factor = factor.clamp(0.0, 1.0);
        Color {
            r: (self.r as f64 * factor + other.r as f64 * (1.0 - factor)) as u8,
            g: (self.g as f64 * factor + other.g as f64 * (1.0 - factor)) as u8,
            b: (self.b as f64 * factor + other.b as f64 * (1.0 - factor)) as u8,
            a: (self.a as f64 * factor + other.a as f64 * (1.0 - factor)) as u8,
        }
    }

    /// Adjust brightness of the color.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount to adjust the brightness (can be negative to darken).
    ///
    /// # Returns
    ///
    /// A new `Color` with adjusted brightness.
    pub fn brighten(&self, amount: i32) -> Color {
        Color {
            r: (self.r as i32 + amount).clamp(0, 255) as u8,
            g: (self.g as i32 + amount).clamp(0, 255) as u8,
            b: (self.b as i32 + amount).clamp(0, 255) as u8,
            a: self.a,
        }
    }

    /// Convert to grayscale.
    ///
    /// # Returns
    ///
    /// A new `Color` representing the grayscale version of this color.
    pub fn grayscale(&self) -> Color {
        let gray = (self.r as f64 * 0.299 + self.g as f64 * 0.587 + self.b as f64 * 0.114) as u8;
        Color {
            r: gray,
            g: gray,
            b: gray,
            a: self.a,
        }
    }

    /// Create a new color with the specified alpha value.
    ///
    /// # Arguments
    ///
    /// * `alpha` - The alpha value (0-255) for the new color.
    ///
    /// # Returns
    ///
    /// A new `Color` with the specified alpha value.
    pub fn with_alpha(&self, alpha: u8) -> Color {
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: alpha,
        }
    }

    /// Interpolate between this color and another color.
    ///
    /// # Arguments
    ///
    /// * `other` - The color to interpolate with.
    /// * `t` - A value between 0.0 and 1.0, where 0.0 returns `self` and 1.0 returns `other`.
    ///
    /// # Returns
    ///
    /// A new `Color` resulting from the interpolation.
    pub fn interpolate(&self, other: Color, t: f64) -> Color {
        Color {
            r: (self.r as f64 * (1.0 - t) + other.r as f64 * t) as u8,
            g: (self.g as f64 * (1.0 - t) + other.g as f64 * t) as u8,
            b: (self.b as f64 * (1.0 - t) + other.b as f64 * t) as u8,
            a: (self.a as f64 * (1.0 - t) + other.a as f64 * t) as u8,
        }
    }

    /// Invert the color.
    ///
    /// # Returns
    ///
    /// A new `Color` that is the inverse of this color.
    pub fn invert(&self) -> Color {
        Color {
            r: 255 - self.r,
            g: 255 - self.g,
            b: 255 - self.b,
            a: self.a,
        }
    }

    /// Convert the color to an RGBA tuple.
    ///
    /// # Returns
    ///
    /// A tuple `(r, g, b, a)` representing the color.
    pub fn to_rgba(&self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }

    /// Create a color from an RGBA tuple.
    ///
    /// # Arguments
    ///
    /// * `rgba` - A tuple containing `(r, g, b, a)`.
    ///
    /// # Returns
    ///
    /// A new `Color` created from the provided RGBA values.
    pub fn from_rgba(rgba: (u8, u8, u8, u8)) -> Color {
        Color {
            r: rgba.0,
            g: rgba.1,
            b: rgba.2,
            a: rgba.3,
        }
    }

    /// Adjust the alpha value of the color.
    ///
    /// # Arguments
    ///
    /// * `alpha` - The new alpha value (0-255).
    ///
    /// # Returns
    ///
    /// A new `Color` with the updated alpha value.
    pub fn adjust_alpha(&self, alpha: i32) -> Color {
        let new_alpha = (self.a as i32 + alpha).clamp(0, 255) as u8;
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: new_alpha,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color3 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color3 {
    /// Blend two colors based on a given factor.
    ///
    /// # Arguments
    ///
    /// * `other` - The color to blend with.
    /// * `factor` - The blend factor (0.0 to 1.0), where 0.0 returns `self` and 1.0 returns `other`.
    ///
    /// # Returns
    ///
    /// A new `Color3` resulting from the blend.
    pub fn blend(&self, other: Color3, factor: f64) -> Color3 {
        let factor = factor.clamp(0.0, 1.0);
        Color3 {
            r: (self.r as f64 * factor + other.r as f64 * (1.0 - factor)) as u8,
            g: (self.g as f64 * factor + other.g as f64 * (1.0 - factor)) as u8,
            b: (self.b as f64 * factor + other.b as f64 * (1.0 - factor)) as u8,
        }
    }

    /// Adjust brightness of the color.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount to adjust the brightness (can be negative to darken).
    ///
    /// # Returns
    ///
    /// A new `Color3` with adjusted brightness.
    pub fn brighten(&self, amount: i32) -> Color3 {
        Color3 {
            r: (self.r as i32 + amount).clamp(0, 255) as u8,
            g: (self.g as i32 + amount).clamp(0, 255) as u8,
            b: (self.b as i32 + amount).clamp(0, 255) as u8,
        }
    }

    /// Convert to grayscale.
    ///
    /// # Returns
    ///
    /// A new `Color3` representing the grayscale version of this color.
    pub fn grayscale(&self) -> Color3 {
        let gray = (self.r as f64 * 0.299 + self.g as f64 * 0.587 + self.b as f64 * 0.114) as u8;
        Color3 {
            r: gray,
            g: gray,
            b: gray,
        }
    }

    /// Create a new color with the specified alpha value.
    ///
    /// # Arguments
    ///
    /// * `alpha` - The alpha value (0-255) for the new color.
    ///
    /// # Returns
    ///
    /// A new `Color` with the specified alpha value.
    pub fn with_alpha(&self, alpha: u8) -> Color {
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: alpha,
        }
    }

    /// Interpolate between this color and another color.
    ///
    /// # Arguments
    ///
    /// * `other` - The color to interpolate with.
    /// * `t` - A value between 0.0 and 1.0, where 0.0 returns `self` and 1.0 returns `other`.
    ///
    /// # Returns
    ///
    /// A new `Color3` resulting from the interpolation.
    pub fn interpolate(&self, other: Color, t: f64) -> Color3 {
        Color3 {
            r: (self.r as f64 * (1.0 - t) + other.r as f64 * t) as u8,
            g: (self.g as f64 * (1.0 - t) + other.g as f64 * t) as u8,
            b: (self.b as f64 * (1.0 - t) + other.b as f64 * t) as u8,
        }
    }

    /// Invert the color.
    ///
    /// # Returns
    ///
    /// A new `Color3` that is the inverse of this color.
    pub fn invert(&self) -> Color3 {
        Color3 {
            r: 255 - self.r,
            g: 255 - self.g,
            b: 255 - self.b,
        }
    }

    /// Convert the color3 to an RGB tuple.
    ///
    /// # Returns
    ///
    /// A tuple `(r, g, b)` representing the color.
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    /// Create a color3 from an RGB tuple.
    ///
    /// # Arguments
    ///
    /// * `rgb` - A tuple containing `(r, g, b)`.
    ///
    /// # Returns
    ///
    /// A new `Color3` created from the provided RGBA values.
    pub fn from_rgb(rgb: (u8, u8, u8)) -> Color3 {
        Color3 {
            r: rgb.0,
            g: rgb.1,
            b: rgb.2,
        }
    }
}
