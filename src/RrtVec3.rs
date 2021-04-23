use std::ops::{Add, Sub, Mul, Div, Index, IndexMut, Neg};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    e:[f64; 3]
}

impl Vec3 {
    pub fn new(e:[f64; 3]) -> Vec3 {
        Vec3 {
            e:e
        }
    }

    pub fn x(&self) -> f64 {
        return self.e[0];
    }

    pub fn y(&self) -> f64 {
        return self.e[1]
    }

    pub fn z(&self) -> f64 {
        return self.e[2]
    }

    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt();
    }

    pub fn length_squared(&self) -> f64 {
        return self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2];
    }

}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    return u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    return Vec3{
        e: [
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0]
        ]
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    return v / v.length();
}

pub fn add(u: Vec3, v: Vec3) -> Vec3 {
    return Vec3 {
        e: [
            u.e[0] + v.e[0],        
            u.e[1] + v.e[1],
            u.e[2] + v.e[2]
        ]
    }
}

pub fn sub(u: Vec3, v: Vec3) -> Vec3 {
    return Vec3 {
        e: [
            u.e[0] - v.e[0],        
            u.e[1] - v.e[1],
            u.e[2] - v.e[2]
        ]
    }
}

pub fn mul(u: Vec3, v: Vec3) -> Vec3 {
    return Vec3 {
        e: [
            u.e[0] * v.e[0],        
            u.e[1] * v.e[1],
            u.e[2] * v.e[2]
        ]
    }
}

impl Add for Vec3 {

    type Output = Self;

    fn add(self, _vec3: Vec3) -> Self {
        Self {
            e:[
                self.e[0] + _vec3.e[0],
                self.e[1] + _vec3.e[1],
                self.e[2] + _vec3.e[2]
            ]
        }
    }
}

impl Sub for Vec3 {

    type Output = Self;

    fn sub(self, _vec3: Self) -> Self {
        Self {
            e:[
                self.e[0] - _vec3.e[0],
                self.e[1] - _vec3.e[1],
                self.e[2] - _vec3.e[2]
            ]
        }
    }
}

impl Mul for Vec3 {

    type Output = Self;

    fn mul(self, t : Vec3) -> Self {
        Self {
            e:[
                self.e[0] * t[0],
                self.e[1] * t[1],
                self.e[2] * t[2]
            ]
        }
    }
}

impl Div<f64> for Vec3 {

    type Output = Self;

    fn div(self, t : f64) -> Self {

        if t == 0.0 {
            panic!("division by zero !");
        }

        Self {
            e:[
                self.e[0] * (1.0/t),
                self.e[1] * (1.0/t),
                self.e[2] * (1.0/t)
            ]
        }
    }
}

impl Index<usize> for Vec3 {

    type Output = f64;

    fn index(&self, index : usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index : usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl Neg for Vec3 {

    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            e:[
                -self.e[0],
                -self.e[1],
                -self.e[2]
            ]
        }
    }
}