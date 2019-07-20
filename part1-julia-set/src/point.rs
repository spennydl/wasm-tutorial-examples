/*
Companion example to the bottoms-up wasm guide.
Copyright (C) 2019  Spencer Leslie <spencerdleslie@gmail.com>

This program is free software; you can redistribute it and/or
modify it under the terms of the GNU General Public License
as published by the Free Software Foundation; either version 2
of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, write to the Free Software
Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
*/

#![deny(missing_docs)]

//!
//! Provides a QOL point struct for the julia set calculation.
//!

/// Point struct.
#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    x: f64,
    y: f64
}

/// Implement the addition operator.
impl std::ops::Add for Point {
    type Output = Self;

    /// Add two points.
    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Point {
    /// Square this point.
    pub fn squared(self) -> Point {
        Point {
            x: self.x.powf(2.0) - self.y.powf(2.0),
            y: 2.0 * self.x * self.y
        }
    }

    /// Get the linear index corresponding to this point's X and Y coords.
    /// Assumes square dimensions!
    pub fn linear_index(&self, dim: usize) -> usize {
        self.y as usize * dim + self.x as usize
    }

    /// Transpose this point from one range to another.
    pub fn to_range(self, min: f64, max: f64, oldmin: f64, oldmax: f64) -> Point {
        Point {
            x: ((self.x - oldmin) * ((max - min) / (oldmax - oldmin))) + min,
            y: ((self.y - oldmin) * ((max - min) / (oldmax - oldmin))) + min,
        }
    }

    /// Calculate the magnitude of this point.
    pub fn magnitude(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
}

/// Convenience from implementation for (f64, f64)
impl From<(f64, f64)> for Point {
    fn from(p: (f64, f64)) -> Self {
        Self { x: p.0, y: p.1 }
    }
}

/// Convenience from implementation for (usize, usize)
impl From<(usize, usize)> for Point {
    fn from(p: (usize, usize)) -> Self {
        Self { x: p.0 as f64, y: p.1 as f64 }
    }
}
