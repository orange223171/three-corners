//! Coordinates definitions

use std::ops::{Add, Sub};

/// A 2D vector
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector {
    /// X coordinate
    pub x: u32,
    /// Y coordinate
    pub y: u32,
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::u32;

    use super::*;

    #[test]
    fn addition() {
        let left = Vector { x: 61, y: 5 };
        let right = Vector { x: 85, y: 90 };

        let result = left + right;
        let correct_result = Vector { x: 146, y: 95 };

        assert_eq!(result, correct_result)
    }

    #[test]
    #[should_panic]
    fn addition_overflow() {
        let left = Vector {
            x: u32::MAX,
            y: u32::MAX,
        };
        let right = Vector { x: 1, y: 1 };

        let _ = left + right;
    }

    #[test]
    fn subtraction() {
        let left = Vector { x: 57, y: 89 };
        let right = Vector { x: 24, y: 87 };

        let result = left - right;
        let correct_result = Vector { x: 33, y: 2 };

        assert_eq!(result, correct_result)
    }

    #[test]
    #[should_panic]
    fn subtraction_overflow() {
        let left = Vector { x: 8, y: 7 };
        let right = Vector { x: 34, y: 8 };

        let _ = left - right;
    }
}
