#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, other: Vec3) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Vec3) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

/*impl std::cmp::Ord for Vec3 {
    fn cmp(first: &Self, other: &Self) -> std::cmp::Ordering {
        let f_val = first.x + first.y + first.z;
        let o_val = first.x + first.y + first.z;

        std::cmp
    }
}
impl std::cmp::PartialOrd for Vec3 {
    fn partial_cmp(first: &Self, other: &Rhs) {}
}*/

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Vec3 {
    pub fn length(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&self) -> Self {
        let l = self.length();

        Vec3::new((self).x / l, (self).y / l, (self).z / l)
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn dot(first: Vec3, other: Vec3) -> f64 {
        first.x * other.x + first.y * other.y + first.z * other.z
    }

    pub fn between(v1: Vec3, v2: Vec3) -> Vec3 {
        Vec3::new(v2.x - v1.x, v2.y - v1.y, v2.z - v1.z)
    }
}
