use std::process::Output;


// Type aliases for vec3
type point3 = vec3;   // 3D point
type color = vec3;    // RGB color

#[derive(Debug)]
struct vec3 {
    e: Vec<f64>,
}

impl vec3 {

    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        vec3 {
            e: vec!(e0, e1, e2)
        }
    }

    pub fn x(self) -> f64 {
        self.e[0]
    }

    pub fn y(self) -> f64 {
        self.e[1]
    }

    pub fn z(self) -> f64 {
        self.e[2]
    }

    pub fn length_squared(self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
}

impl std::ops::AddAssign for vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: vec!(
                self.e[0] + other.e[0], 
                self.e[1] + other.e[1], 
                self.e[2] + other.e[2], 
            ),
        }
    }
}

impl std::ops::MulAssign<f64> for vec3 {
    fn mul_assign(&mut self, t: f64) {
        *self = Self {
            e: vec!(
                self.e[0] * t,
                self.e[1] * t,
                self.e[2] * t,
            ),
        }
    }
}

//     double operator[](int i) const { return e[i]; }
impl std::ops::Neg for vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            e: vec!(
                -self.e[0],
                -self.e[1],
                -self.e[2],
            )
        }
    }
}

impl std::ops::Div<f64> for vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            e: vec!(
                self.e[0] / rhs,
                self.e[1] / rhs,
                self.e[2] / rhs,
            )

        }
    }
}

//     vec3& operator/=(const double t) {
//         return *this *= 1/t;
//     }
impl std::ops::DivAssign<f64> for vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self {
            e: vec!(
                self.e[0] / rhs,
                self.e[1] / rhs,
                self.e[2] / rhs,
            )

        }
    }
}

// this seems like a waste of code
//     double& operator[](int i) { return e[i]; }




