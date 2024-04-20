use crate::{Result, Vector};

impl std::ops::Add for &Vector {
    type Output = Result<Vector>;

    fn add(self, rhs: Self) -> Self::Output {
        let inner = self
            .0
            .iter()
            .zip(rhs.0.iter())
            .map(|(x, y)| x + y)
            .collect();

        Ok(Vector(inner))
    }
}

impl std::ops::Add<&Self> for Vector {
    type Output = Result<Vector>;

    fn add(self, rhs: &Self) -> Self::Output {
        &self + rhs
    }
}

impl std::ops::Add<f32> for &Vector {
    type Output = Result<Vector>;

    fn add(self, rhs: f32) -> Self::Output {
        let inner = self.0.iter().map(|x| x + rhs).collect();

        Ok(Vector(inner))
    }
}

impl std::ops::Add<f32> for Vector {
    type Output = Result<Vector>;

    fn add(self, rhs: f32) -> Self::Output {
        &self + rhs
    }
}

impl std::ops::AddAssign<&Vector> for &mut Vector {
    fn add_assign(&mut self, rhs: &Vector) {
        self.0
            .iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(x, y)| *x += y);
    }
}

impl std::ops::AddAssign<&Vector> for Vector {
    fn add_assign(&mut self, rhs: &Vector) {
        let mut s = self;
        s += rhs;
    }
}

impl std::ops::AddAssign<f32> for &mut Vector {
    fn add_assign(&mut self, rhs: f32) {
        self.0.iter_mut().for_each(|x| *x += rhs);
    }
}

impl std::ops::AddAssign<f32> for Vector {
    fn add_assign(&mut self, rhs: f32) {
        let mut s = self;
        s += rhs;
    }
}

impl std::ops::Sub for &Vector {
    type Output = Result<Vector>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.validate_dimensions(&rhs)?;

        let inner = self
            .0
            .iter()
            .zip(rhs.0.iter())
            .map(|(x, y)| x - y)
            .collect();

        Ok(Vector(inner))
    }
}

impl std::ops::Sub<&Vector> for Vector {
    type Output = Result<Self>;

    fn sub(self, rhs: &Self) -> Self::Output {
        &self - rhs
    }
}

impl std::ops::Sub<f32> for &Vector {
    type Output = Vector;

    fn sub(self, rhs: f32) -> Self::Output {
        let inner = self.0.iter().map(|x| x - rhs).collect();

        Vector(inner)
    }
}

impl std::ops::Sub<f32> for Vector {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        let inner = self.0.iter().map(|x| x - rhs).collect();

        Self(inner)
    }
}

impl std::ops::SubAssign<&Vector> for &mut Vector {
    fn sub_assign(&mut self, rhs: &Vector) {
        self.0
            .iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(x, y)| *x -= y);
    }
}

impl std::ops::SubAssign<&Vector> for Vector {
    fn sub_assign(&mut self, rhs: &Vector) {
        let mut s = self;
        s -= rhs;
    }
}

impl std::ops::SubAssign<f32> for &mut Vector {
    fn sub_assign(&mut self, rhs: f32) {
        self.0.iter_mut().for_each(|x| *x -= rhs);
    }
}

impl std::ops::SubAssign<f32> for Vector {
    fn sub_assign(&mut self, rhs: f32) {
        let mut s = self;
        s -= rhs;
    }
}

impl std::ops::Mul<f32> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        let inner = self.0.iter().map(|x| x * rhs).collect();

        Vector(inner)
    }
}

impl std::ops::Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        &self * rhs
    }
}

impl std::ops::Mul<&Vector> for &Vector {
    type Output = Result<f32>;

    fn mul(self, rhs: &Vector) -> Self::Output {
        self.dot(rhs)
    }
}

impl std::ops::Mul<&Vector> for Vector {
    type Output = Result<f32>;

    fn mul(self, rhs: &Vector) -> Self::Output {
        self.as_ref() * rhs
    }
}

impl std::ops::MulAssign<&Vector> for &mut Vector {
    fn mul_assign(&mut self, rhs: &Vector) {
        self.0
            .iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(x, y)| *x *= y);
    }
}

impl std::ops::MulAssign<&Vector> for Vector {
    fn mul_assign(&mut self, rhs: &Vector) {
        let mut s = self;
        s *= rhs;
    }
}

impl std::ops::MulAssign<f32> for &mut Vector {
    fn mul_assign(&mut self, rhs: f32) {
        self.0.iter_mut().for_each(|x| *x *= rhs);
    }
}

impl std::ops::MulAssign<f32> for Vector {
    fn mul_assign(&mut self, rhs: f32) {
        let mut s = self;
        s *= rhs;
    }
}
