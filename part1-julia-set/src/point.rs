
#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    x: f64,
    y: f64
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Point {
    pub fn squared(self) -> Point {
        Point {
            x: self.x.powf(2.0) - self.y.powf(2.0),
            y: 2.0 * self.x * self.y
        }
    }

    pub fn linear_index(&self, dim: usize) -> usize {
        self.y as usize * dim + self.x as usize
    }

    pub fn to_range(self, min: f64, max: f64, oldmin: f64, oldmax: f64) -> Point {
        Point {
            x: ((self.x - oldmin) * ((max - min) / (oldmax - oldmin))) + min,
            y: ((self.y - oldmin) * ((max - min) / (oldmax - oldmin))) + min,
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
}

impl From<(f64, f64)> for Point {
    fn from(p: (f64, f64)) -> Self {
        Self { x: p.0, y: p.1 }
    }
}

impl From<(usize, usize)> for Point {
    fn from(p: (usize, usize)) -> Self {
        Self { x: p.0 as f64, y: p.1 as f64 }
    }
}
