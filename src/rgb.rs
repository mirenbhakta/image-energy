use std::ops::{Add, Sub, Mul, Div};

#[derive(Copy, Clone)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Rgb {
    pub fn from_u8(rgb: &[u8]) -> Self {
        Self {
            r: rgb[0] as f32 / 255.0,
            g: rgb[1] as f32 / 255.0,
            b: rgb[2] as f32 / 255.0,
        }
    }

    pub fn to_u8(&self) -> [u8; 3] {
        [(self.r * 255.0) as u8, (self.g * 255.0) as u8, (self.b * 255.0) as u8]
    }

    pub fn squared_distance(&self, other: &Self) -> f32 {
        (self.r - other.r).powi(2) + (self.g - other.g).powi(2) + (self.b - other.b).powi(2)
    }

    pub fn distance(&self, other: &Self) -> f32 {
        self.squared_distance(other).sqrt()
    }

    pub fn squared_len(&self) -> f32 {
        self.r.powi(2) + self.g.powi(2) + self.b.powi(2)
    }

    pub fn len(&self) -> f32 {
        self.squared_len().sqrt()
    }

    pub fn sqrt_each(&self) -> Self {
        Self {
            r: self.r.sqrt(),
            g: self.g.sqrt(),
            b: self.b.sqrt(),
        }
    }

    pub fn abs_each(&self) -> Self {
        Self {
            r: self.r.abs(),
            g: self.g.abs(),
            b: self.b.abs(),
        }
    }
}

impl Add<Rgb> for Rgb {
    type Output = Rgb;

    fn add(self, rhs: Self) -> Rgb {
        Rgb {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl<'a> Add<&'a Rgb> for &'a Rgb {
    type Output = Rgb;

    fn add(self, rhs: Self) -> Rgb {
        Rgb {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Sub<Rgb> for Rgb {
    type Output = Rgb;

    fn sub(self, rhs: Self) -> Rgb {
        Rgb {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl<'a> Sub<&'a Rgb> for &'a Rgb {
    type Output = Rgb;

    fn sub(self, rhs: Self) -> Rgb {
        Rgb {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl Mul<Rgb> for Rgb {
    type Output = Rgb;

    fn mul(self, rhs: Self) -> Rgb {
        Rgb {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl<'a> Mul<&'a Rgb> for &'a Rgb {
    type Output = Rgb;

    fn mul(self, rhs: Self) -> Rgb {
        Rgb {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Mul<f32> for Rgb {
    type Output = Rgb;

    fn mul(self, rhs: f32) -> Self {
        Rgb {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl<'a> Mul<f32> for &'a Rgb {
    type Output = Rgb;

    fn mul(self, rhs: f32) -> Rgb {
        Rgb {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Div<Rgb> for Rgb {
    type Output = Rgb;

    fn div(self, rhs: Self) -> Rgb {
        Rgb {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
        }
    }
}

impl<'a> Div<&'a Rgb> for &'a Rgb {
    type Output = Rgb;

    fn div(self, rhs: Self) -> Rgb {
        Rgb {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
        }
    }
}

impl Div<f32> for Rgb {
    type Output = Rgb;

    fn div(self, rhs: f32) -> Self {
        Rgb {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

impl<'a> Div<f32> for &'a Rgb {
    type Output = Rgb;

    fn div(self, rhs: f32) -> Rgb {
        Rgb {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}
