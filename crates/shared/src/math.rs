
use std::ops::{Add, Sub, Mul};

#[derive(Debug, Copy, Clone, PartialEq,Default)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Vec2f {

    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn length_sq(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn normalized(self) -> Self {
        let len = self.length();
        if len > 0.0001 {
            self * (1.0 / len)
        } else {
            Self::ZERO
        }
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }

    pub fn distance_sq(self, other: Self) -> f32 {
        (self - other).length_sq()
    }

    pub fn lerp(self, target: Self, t: f32) -> Self {
        self + (target - self) * t.clamp(0.0, 1.0)
    }

    pub fn angle(self) -> f32 {
        self.y.atan2(self.x)
    }

    pub fn from_angle(angle: f32) -> Self {
        Self {
            x: angle.cos(),
            y: angle.sin(),
        }
    }

    pub fn clamp_length(self, max: f32) -> Self {
        let len_sq = self.length_sq();
        if len_sq > max * max {
            self.normalized() * max
        } else {
            self
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Vec2i {
    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }


    pub fn length(self) -> i32 {
        (self.x * self.x + self.y * self.y).isqrt()
    }

    pub fn length_sq(self) -> i32 {
        self.x * self.x + self.y * self.y
    }

    pub fn dot(self, other: Self) -> i32 {
        self.x * other.x + self.y * other.y
    }


    pub fn distance(self, other: Self) -> i32 {
        (self - other).length()
    }

    pub fn distance_sq(self, other: Self) -> i32 {
        (self - other).length_sq()
    }



    pub fn neighbors_4(self) -> [Self; 4] {
        [
            Self::new(self.x + 1, self.y),
            Self::new(self.x - 1, self.y),
            Self::new(self.x, self.y + 1),
            Self::new(self.x, self.y - 1),
        ]
    }

    pub fn neighbors_8(self) -> [Self; 8] {
        [
            Self::new(self.x + 1, self.y),
            Self::new(self.x - 1, self.y),
            Self::new(self.x, self.y + 1),
            Self::new(self.x, self.y - 1),
            Self::new(self.x + 1, self.y + 1),
            Self::new(self.x - 1, self.y - 1),
            Self::new(self.x + 1, self.y - 1),
            Self::new(self.x - 1, self.y + 1),
        ]
    }

    pub fn to_f32(self) -> Vec2f {
        Vec2f::new(self.x as f32, self.y as f32)
    }
}





macro_rules! impl_vec2_ops {
    ($t:ty, $scalar:ty) => {
        impl Add for $t {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                Self {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }

        impl Sub for $t {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
                Self {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                }
            }
        }

        impl Mul<$scalar> for $t {
            type Output = Self;
            fn mul(self, rhs: $scalar) -> Self {
                Self {
                    x: self.x * rhs,
                    y: self.y * rhs,
                }
            }
        }
    };
}

impl_vec2_ops!(Vec2f, f32);
impl_vec2_ops!(Vec2i, i32);