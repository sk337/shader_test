use std::ops::*;

/// A struct representing a point in 2D space.
#[derive(Debug, Clone, Copy)]
pub struct Point {
    /// The x-coordinate of the point.
    pub x: f64,
    /// The y-coordinate of the point.
    pub y: f64,
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Point) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, scalar: f64) -> Point {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl MulAssign<f64> for Point {
    fn mul_assign(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, scalar: f64) -> Point {
        Point {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl DivAssign<f64> for Point {
    fn div_assign(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

impl Mul<Point> for Point {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl MulAssign<Point> for Point {
    fn mul_assign(&mut self, other: Point) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl Div<Point> for Point {
    type Output = Point;

    fn div(self, other: Point) -> Point {
        Point {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl DivAssign<Point> for Point {
    fn div_assign(&mut self, other: Point) {
        self.x /= other.x;
        self.y /= other.y;
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Point {
    /// Creates a new `Point` with the given x and y coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the point.
    /// * `y` - The y-coordinate of the point.
    ///
    /// # Returns
    ///
    /// A new `Point` instance.
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    /// Calculates the distance between this point and another point.
    ///
    /// # Arguments
    ///
    /// * `other` - The other point to calculate the distance to.
    ///
    /// # Returns
    ///
    /// The Euclidean distance between the two points.
    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    /// Calculates the magnitude (length) of the vector from the origin to this point.
    ///
    /// # Returns
    ///
    /// The magnitude of the point as a vector.
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    /// Returns a normalized version of this point, which is a vector with the same
    /// direction but a magnitude of 1.
    ///
    /// # Returns
    ///
    /// A new `Point` representing the normalized vector.
    pub fn normalize(&self) -> Point {
        let mag = self.magnitude();
        if mag == 0.0 {
            *self
        } else {
            *self / mag
        }
    }

    /// Calculates the dot product of this point with another point.
    ///
    /// # Arguments
    ///
    /// * `other` - The other point to calculate the dot product with.
    ///
    /// # Returns
    ///
    /// The dot product of the two points.
    pub fn dot(&self, other: &Point) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// Calculates the midpoint between this point and another point.
    ///
    /// # Arguments
    ///
    /// * `other` - The other point to calculate the midpoint with.
    ///
    /// # Returns
    ///
    /// A new `Point` representing the midpoint.
    pub fn midpoint(&self, other: &Point) -> Point {
        Point {
            x: (self.x + other.x) / 2.0,
            y: (self.y + other.y) / 2.0,
        }
    }

    /// Checks if this point is near another point within a specified tolerance.
    ///
    /// # Arguments
    ///
    /// * `other` - The other point to compare with.
    /// * `tolerance` - The maximum distance between the points to consider them as near.
    ///
    /// # Returns
    ///
    /// `true` if the points are within the specified tolerance, otherwise `false`.
    pub fn is_near(&self, other: &Point, tolerance: f64) -> bool {
        self.distance(other) <= tolerance
    }
}
