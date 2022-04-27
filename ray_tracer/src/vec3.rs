
// Type aliases for vec3
type point3 = vec3;   // 3D point
type color = vec3;    // RGB color

#[derive(Debug)]
struct vec3 {
    e: vec<f64>,
}

impl vec3 {

    pub fn new() -> vec3 {
        vec3{
            e: vec!(0,0,0)
        }
    }

    pub fn new(e0: f64, e1: f64, e2: f64) {
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

// public:

//     vec3 operator-() const { return vec3(-e[0], -e[1], -e[2]); }
//     double operator[](int i) const { return e[i]; }
//     double& operator[](int i) { return e[i]; }

//     vec3& operator/=(const double t) {
//         return *this *= 1/t;
//     }



