use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub const ONE: Self = Self {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalized(self) -> Self {
        self / self.length()
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        other * self
    }
}

impl Mul for Vector3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Vector3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Div<Vector3> for f64 {
    type Output = Vector3;

    fn div(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self / other.x,
            y: self / other.y,
            z: self / other.z,
        }
    }
}

impl Div for Vector3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self * (1.0 / other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_new() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_vector_dot() {
        let v1 = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vector3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let result = v1.dot(v2);
        assert_eq!(result, 32.0);
    }

    #[test]
    fn test_vector_cross() {
        let v1 = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vector3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let result = v1.cross(v2);
        assert_eq!(result.x, -3.0);
        assert_eq!(result.y, 6.0);
        assert_eq!(result.z, -3.0);
    }

    #[test]
    fn test_vector_length() {
        let v = Vector3 {
            x: 3.0,
            y: 4.0,
            z: 0.0,
        };
        assert_eq!(v.length(), 5.0);
    }

    #[test]
    fn test_vector_normalized() {
        let v = Vector3 {
            x: 3.0,
            y: 4.0,
            z: 0.0,
        };
        let normalized = v.normalized();
        assert_eq!(normalized.x, 0.6);
        assert_eq!(normalized.y, 0.8);
        assert_eq!(normalized.z, 0.0);
        assert_eq!(normalized.length(), 1.0);
    }

    #[test]
    fn test_vector_add() {
        let v1 = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vector3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let result = v1 + v2;
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 7.0);
        assert_eq!(result.z, 9.0);
    }

    #[test]
    fn test_vector_sub() {
        let v1 = Vector3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let v2 = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let result = v1 - v2;
        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, 3.0);
        assert_eq!(result.z, 3.0);
    }

    #[test]
    fn test_vector_mul() {
        let v = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let scalar = 2.0;

        // Vector * scalar
        let result1 = v * scalar;
        assert_eq!(result1.x, 2.0);
        assert_eq!(result1.y, 4.0);
        assert_eq!(result1.z, 6.0);

        // scalar * Vector
        let result2 = scalar * v;
        assert_eq!(result2.x, 2.0);
        assert_eq!(result2.y, 4.0);
        assert_eq!(result2.z, 6.0);

        // Vector * Vector
        let result3 = v * v;
        assert_eq!(result3.x, 1.0);
        assert_eq!(result3.y, 4.0);
        assert_eq!(result3.z, 9.0);
    }

    #[test]
    fn test_vector_div() {
        let v = Vector3 {
            x: 2.0,
            y: 4.0,
            z: 6.0,
        };
        let scalar = 2.0;

        // Vector / scalar
        let result1 = v / scalar;
        assert_eq!(result1.x, 1.0);
        assert_eq!(result1.y, 2.0);
        assert_eq!(result1.z, 3.0);

        // scalar / Vector
        let result2 = scalar / v;
        assert_eq!(result2.x, 1.0);
        assert_eq!(result2.y, 0.5);
        assert_eq!(result2.z, 1.0 / 3.0);

        // Vector / Vector
        let result3 = v / v;
        assert_eq!(result3.x, 1.0);
        assert_eq!(result3.y, 1.0);
        assert_eq!(result3.z, 1.0);
    }
}
