use std::ops::{Add, Sub, Mul, Div};

#[derive(Copy, Clone)]
pub struct Lab{
    pub l: f32,
    pub a: f32,
    pub b: f32
}

impl Lab {
    pub fn from_rgb(rgb: &[u8; 3]) -> Self {
        let lab = lab::Lab::from_rgb(rgb);
        Self {
            l: lab.l,
            a: lab.a,
            b: lab.b,
        }
    }

    pub fn from_rgba(rgba: &[u8; 4]) -> Self {
        let lab = lab::Lab::from_rgba(rgba);
        Self {
            l: lab.l,
            a: lab.a,
            b: lab.b,
        }
    }

    pub fn to_rgb(&self) -> [u8; 3] {
        let lab = lab::Lab { l: self.l, a: self.a, b: self.b };
        lab.to_rgb()
    }

    pub fn squared_distance(&self, other: &Self) -> f32 {
        let lab = lab::Lab { l: self.l, a: self.a, b: self.b };
        let oth = lab::Lab { l: other.l, a: other.a, b: other.b };
        lab.squared_distance(&oth)
    }

    pub fn distance(&self, other: &Self) -> f32 {
        self.squared_distance(other).sqrt()
    }

    pub fn squared_len(&self) -> f32 {
        (self.l.powi(2) + self.a.powi(2) + self.b.powi(2))
    }

    pub fn len(&self) -> f32 {
        self.squared_len().sqrt()
    }

    pub fn sqrt_each(&self) -> Self {
        Self {
            l: self.l.sqrt(),
            a: self.a.sqrt(),
            b: self.b.sqrt(),
        }
    }

    pub fn abs_each(&self) -> Self {
        Self {
            l: self.l.abs(),
            a: self.a.abs(),
            b: self.b.abs(),
        }
    }

    pub fn to_array(&self) -> [f32; 3] {
        [self.l, self.a, self.b]
    }

    pub fn array_to_rgb(array: &[f32; 3]) -> [u8; 3] {
        let lab = lab::Lab { l: array[0], a: array[1], b: array[2] };
        lab.to_rgb()
    }
}

impl Add<Lab> for Lab {
    type Output = Lab;

    fn add(self, rhs: Self) -> Lab {
        Lab {
            l: self.l + rhs.l,
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}

impl<'a> Add<&'a Lab> for &'a Lab {
    type Output = Lab;
    fn add(self, rhs: Self) -> Lab {
        Lab {
            l: self.l + rhs.l,
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}

impl Sub<Lab> for Lab {
    type Output = Lab;

    fn sub(self, rhs: Self) -> Self {
        Lab {
            l: self.l - rhs.l,
            a: self.a - rhs.a,
            b: self.b - rhs.b,
        }
    }
}

impl<'a> Sub<&'a Lab> for &'a Lab {
    type Output = Lab;

    fn sub(self, rhs: Self) -> Lab {
        Lab {
            l: self.l - rhs.l,
            a: self.a - rhs.a,
            b: self.b - rhs.b,
        }
    }
}

impl Mul<Lab> for Lab {
    type Output = Lab;

    fn mul(self, rhs: Self) -> Lab {
        Lab {
            l: self.l * rhs.l,
            a: self.a * rhs.a,
            b: self.b * rhs.b,
        }
    }
}

impl<'a> Mul<&'a Lab> for &'a Lab {
    type Output = Lab;

    fn mul(self, rhs: Self) -> Lab {
        Lab {
            l: self.l * rhs.l,
            a: self.a * rhs.a,
            b: self.b * rhs.b,
        }
    }
}

impl Mul<f32> for Lab {
    type Output = Lab;

    fn mul(self, rhs: f32) -> Lab {
        Lab {
            l: self.l * rhs,
            a: self.a * rhs,
            b: self.b * rhs,
        }
    }
}

impl<'a> Mul<f32> for &'a Lab {
    type Output = Lab;
    fn mul(self, rhs: f32) -> Lab {
        Lab {
            l: self.l * rhs,
            a: self.a * rhs,
            b: self.b * rhs,
        }
    }
}

impl Div<Lab> for Lab {
    type Output = Lab;

    fn div(self, rhs: Self) -> Lab {
        Lab {
            l: self.l / rhs.l,
            a: self.a / rhs.a,
            b: self.b / rhs.b,
        }
    }
}

impl<'a> Div<&'a Lab> for &'a Lab {
    type Output = Lab;

    fn div(self, rhs: Self) -> Lab {
        Lab {
            l: self.l / rhs.l,
            a: self.a / rhs.a,
            b: self.b / rhs.b,
        }
    }
}

impl Div<f32> for Lab {
    type Output = Lab;

    fn div(self, rhs: f32) -> Lab {
        Lab {
            l: self.l / rhs,
            a: self.a / rhs,
            b: self.b / rhs,
        }
    }
}

impl<'a> Div<f32> for &'a Lab {
    type Output = Lab;
    fn div(self, rhs: f32) -> Lab {
        Lab {
            l: self.l / rhs,
            a: self.a / rhs,
            b: self.b / rhs,
        }
    }
}
