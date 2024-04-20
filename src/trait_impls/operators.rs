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
