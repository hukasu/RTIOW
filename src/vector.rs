use std::ops::Neg;

macro_rules! make_vector3d_type {
    ($type_name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $type_name {
            x: f64,
            y: f64,
            z: f64,
        }

        impl $type_name {
            #[must_use]
            pub fn new(x: f64, y: f64, z: f64) -> Self {
                Self { x, y, z }
            }

            pub fn is_zero(&self) -> bool {
                self.x.abs() < f64::EPSILON
                    && self.y.abs() < f64::EPSILON
                    && self.z.abs() < f64::EPSILON
            }
        }

        impl Default for $type_name {
            fn default() -> Self {
                Self::new(0., 0., 0.)
            }
        }

        impl std::ops::Index<usize> for $type_name {
            type Output = f64;

            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    0 => &self.x,
                    1 => &self.y,
                    2 => &self.z,
                    _ => {
                        panic!(
                            "Index out of bounds. {} always only have 3 indexes (0..=2).",
                            std::stringify!(type_name)
                        )
                    }
                }
            }
        }

        impl std::ops::IndexMut<usize> for $type_name {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    0 => &mut self.x,
                    1 => &mut self.y,
                    2 => &mut self.z,
                    _ => {
                        panic!(
                            "Index out of bounds. {} always only have 3 indexes (0..=2).",
                            std::stringify!(type_name)
                        )
                    }
                }
            }
        }

        impl std::ops::Add for $type_name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                    z: self.z + rhs.z,
                }
            }
        }

        impl std::ops::Add<f64> for $type_name {
            type Output = Self;

            fn add(self, rhs: f64) -> Self::Output {
                Self {
                    x: self.x + rhs,
                    y: self.y + rhs,
                    z: self.z + rhs,
                }
            }
        }

        impl std::ops::AddAssign for $type_name {
            fn add_assign(&mut self, rhs: Self) {
                self.x += rhs.x;
                self.y += rhs.y;
                self.z += rhs.z;
            }
        }

        impl std::ops::Sub for $type_name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                    z: self.z - rhs.z,
                }
            }
        }

        impl std::ops::Mul for $type_name {
            type Output = Self;

            fn mul(self, rhs: $type_name) -> Self::Output {
                Self {
                    x: self.x * rhs.x,
                    y: self.y * rhs.y,
                    z: self.z * rhs.z,
                }
            }
        }

        impl std::ops::Mul<f64> for $type_name {
            type Output = Self;

            fn mul(self, rhs: f64) -> Self::Output {
                Self {
                    x: self.x * rhs,
                    y: self.y * rhs,
                    z: self.z * rhs,
                }
            }
        }

        impl std::ops::Mul<$type_name> for f64 {
            type Output = $type_name;

            fn mul(self, rhs: Self::Output) -> Self::Output {
                Self::Output {
                    x: self * rhs.x,
                    y: self * rhs.y,
                    z: self * rhs.z,
                }
            }
        }

        impl std::ops::MulAssign<f64> for $type_name {
            fn mul_assign(&mut self, rhs: f64) {
                self.x *= rhs;
                self.y *= rhs;
                self.z *= rhs;
            }
        }

        impl std::ops::Div<f64> for $type_name {
            type Output = Self;

            fn div(self, rhs: f64) -> Self::Output {
                Self {
                    x: self.x / rhs,
                    y: self.y / rhs,
                    z: self.z / rhs,
                }
            }
        }

        impl std::ops::Div<$type_name> for f64 {
            type Output = $type_name;

            fn div(self, rhs: Self::Output) -> Self::Output {
                Self::Output {
                    x: self / rhs.x,
                    y: self / rhs.y,
                    z: self / rhs.z,
                }
            }
        }

        impl std::ops::DivAssign<f64> for $type_name {
            fn div_assign(&mut self, rhs: f64) {
                self.x /= rhs;
                self.y /= rhs;
                self.z /= rhs;
            }
        }

        impl std::ops::Neg for $type_name {
            type Output = Self;

            fn neg(self) -> Self::Output {
                self * -1.
            }
        }
    };
}

make_vector3d_type!(Point);
make_vector3d_type!(Direction);
make_vector3d_type!(Colour);

impl std::ops::Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Point {
    pub fn point_towards(self, rhs: Point) -> Direction {
        Direction {
            x: rhs.x - self.x,
            y: rhs.y - self.y,
            z: rhs.z - self.z,
        }
    }
}

impl Direction {
    /// Creates a Direction with each component between [-1.0..1.0).
    #[must_use]
    pub fn new_random() -> Self {
        Self {
            x: rand::random::<f64>() * 2. - 1.,
            y: rand::random::<f64>() * 2. - 1.,
            z: rand::random::<f64>() * 2. - 1.,
        }
    }

    /// Creates a Direction with length squared smaller than 1.
    #[must_use]
    pub fn new_random_in_unit_sphere() -> Self {
        loop {
            let rand_vec = Self {
                x: rand::random::<f64>() * 2. - 1.,
                y: rand::random::<f64>() * 2. - 1.,
                z: rand::random::<f64>() * 2. - 1.,
            };
            if rand_vec.length_squared() < 1. {
                break rand_vec;
            }
        }
    }

    #[must_use]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    #[must_use]
    pub fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    #[must_use]
    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[must_use]
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    #[must_use]
    pub fn unit_vector(self) -> Self {
        self / self.length()
    }

    #[must_use]
    pub fn random_direction_in_hemisphere(&self) -> Self {
        let on_unit_sphere = Self::new_random_in_unit_sphere().unit_vector();
        if on_unit_sphere.dot(*self) > 0. {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    #[must_use]
    pub fn reflect(&self, normal: &Self) -> Self {
        *self - 2. * self.dot(*normal) * *normal
    }

    #[must_use]
    pub fn refract(&self, normal: &Self, refraction_ratio: f64) -> Self {
        //     auto cos_theta = fmin(dot(-uv, n), 1.0);
        // vec3 r_out_perp =  etai_over_etat * (uv + cos_theta*n);
        // vec3 r_out_parallel = -sqrt(fabs(1.0 - r_out_perp.length_squared())) * n;
        // return r_out_perp + r_out_parallel;
        let cos_theta = (-*self).dot(*normal).min(1.);
        let perpendicular_direction = refraction_ratio * (*self + cos_theta * *normal);
        let parallel_direction = *normal
            * (1. - perpendicular_direction.length_squared())
                .abs()
                .sqrt()
                .neg();
        perpendicular_direction + parallel_direction
    }
}

impl Colour {
    /// Creates a Colour with each component between [0.0..1.0).
    #[must_use]
    pub fn new_random() -> Self {
        Self {
            x: rand::random::<f64>(),
            y: rand::random::<f64>(),
            z: rand::random::<f64>(),
        }
    }

    pub fn linear_to_gamma(self) -> Self {
        Self {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
            z: self.z.sqrt(),
        }
    }
}
