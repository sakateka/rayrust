use rand::{thread_rng, Rng};
use std::fmt::{self, Display};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Range, Sub, SubAssign,
};

#[derive(Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(self) -> f64 {
        self[0]
    }

    pub fn y(self) -> f64 {
        self[1]
    }

    pub fn z(self) -> f64 {
        self[2]
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self[1] * other[2] - self[2] * other[1],
                self[2] * other[0] - self[0] * other[2],
                self[0] * other[1] - self[1] * other[0],
            ],
        }
    }

    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }

    pub fn near_zero(self) -> bool {
        const EPS: f64 = 1.0e-8;
        self[0].abs() < EPS && self[1].abs() < EPS && self[2].abs() < EPS
    }

    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - 2.0 * self.dot(n) * n
    }

    pub fn refract(self, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-self).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * n;
        r_out_perp + r_out_parallel
    }

    pub fn format_color(self, samples_per_pixel: u64) -> String {
        let scale = 1.0 / samples_per_pixel as f64;
        let ir = (256.0 * (self[0] * scale).sqrt().clamp(0.0, 0.999)) as u64;
        let ig = (256.0 * (self[1] * scale).sqrt().clamp(0.0, 0.999)) as u64;
        let ib = (256.0 * (self[2] * scale).sqrt().clamp(0.0, 0.999)) as u64;
        format!("{} {} {}", ir, ig, ib)
    }

    pub fn random(r: Range<f64>) -> Vec3 {
        let mut rng = thread_rng();
        Vec3 {
            e: [
                rng.gen_range(r.clone()),
                rng.gen_range(r.clone()),
                rng.gen_range(r),
            ],
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Self::random_in_unit_sphere().normalized()
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let v = Vec3::random(-1.0..1.0);
            if v.length_squared() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.e[index]
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z(),
            ],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) -> () {
        *self = Vec3 {
            e: [
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z(),
            ],
        };
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.x() - other.x(),
                self.y() - other.y(),
                self.z() - other.z(),
            ],
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) -> () {
        *self = Vec3 {
            e: [
                self.x() - other.x(),
                self.y() - other.y(),
                self.z() - other.z(),
            ],
        };
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            e: [self.x() * other, self.y() * other, self.z() * other],
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) -> () {
        *self = Vec3 {
            e: [self.x() * other, self.y() * other, self.z() * other],
        };
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.x() * other.x(),
                self.y() * other.y(),
                self.z() * other.z(),
            ],
        }
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self * other.x(), self * other.y(), self * other.z()],
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.x(), -self.y(), -self.z()],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3 {
            e: [self.x() / other, self.y() / other, self.z() / other],
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) -> () {
        *self = Vec3 {
            e: [self.x() / other, self.y() / other, self.z() / other],
        };
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}
