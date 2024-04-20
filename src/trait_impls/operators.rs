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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::init_vectors;

    #[test]
    fn can_calculate_vector_addition() {
        let (u, v, _, _) = init_vectors();

        let res = &u + &v;

        let expected = Vector::from_iter(u.0.iter().zip(v.0.iter()).map(|(x, y)| x + y));

        assert_eq!(res, Ok(expected));
    }

    #[test]
    fn can_calculate_vector_addition_with_number() {
        let (u, _, _, _) = init_vectors();

        let res = &u + 1.;
        let expected = Vector::from_iter(u.0.iter().map(|x| x + 1.));
        assert_eq!(res, Ok(expected));
    }

    #[test]
    fn can_add_and_assign_in_vector_addition() {
        let (mut u, v, _, _) = init_vectors();

        let expected = Vector::from_iter(u.0.iter().zip(v.0.iter()).map(|(x, y)| x + y));

        u += &v;

        assert_eq!(u, expected);
    }

    #[test]
    fn can_add_and_assign_in_vector_addition_with_number() {
        let (mut u, _, _, _) = init_vectors();

        let expected = Vector::from_iter(u.0.iter().map(|x| x + 1.));

        u += 1.;

        assert_eq!(u, expected);
    }

    #[test]
    fn can_calculate_vector_subtraction() {
        let (u, v, _, _) = init_vectors();

        let res = &u - &v;

        let expected = Vector::from_iter(u.0.iter().zip(v.0.iter()).map(|(x, y)| x - y));

        assert_eq!(res, Ok(expected));
    }

    #[test]
    fn can_calculate_vector_subtraction_with_number() {
        let (u, _, _, _) = init_vectors();

        let res = &u - 1.;
        let expected = Vector::from_iter(u.0.iter().map(|x| x - 1.));

        assert_eq!(res, expected);
    }

    #[test]
    fn can_subtract_and_assign_in_vector_subtraction() {
        let (mut u, v, _, _) = init_vectors();

        let expected = Vector::from_iter(u.0.iter().zip(v.0.iter()).map(|(x, y)| x - y));

        u -= &v;

        assert_eq!(u, expected);
    }

    #[test]
    fn can_substract_assign_in_vector_subtraction_with_number() {
        let (mut u, _, _, _) = init_vectors();

        let expected = Vector::from_iter(u.0.iter().map(|x| x - 1.));

        u -= 1.;

        assert_eq!(u, expected);
    }

    #[test]
    fn can_calculate_vector_multiplication() {
        let (u, v, _, _) = init_vectors();

        let res = &u * &v;

        assert_eq!(res, Ok(4.));
    }

    #[test]
    fn can_calculate_vector_multiplication_with_number() {
        let (u, _, _, _) = init_vectors();

        let res = &u * 2.;

        let expected = Vector::from_iter(u.0.iter().map(|x| x * 2.));

        assert_eq!(res, expected);
    }

    #[test]
    fn can_multiply_and_assign_in_vector_multiplication() {
        let (mut u, v, _, _) = init_vectors();

        let expected = Vector::from_iter(u.0.iter().zip(v.0.iter()).map(|(x, y)| x * y));

        u *= &v;

        assert_eq!(u, expected);
    }

    #[test]
    fn can_multiply_assign_in_vector_multiplication_with_number() {
        let (mut u, _, _, _) = init_vectors();

        let expected = Vector::from_iter(u.0.iter().map(|x| x * 1.));

        u *= 1.;

        assert_eq!(u, expected);
    }

    #[test]
    fn can_calculate_square() {
        let (u, _, _, _) = init_vectors();
        let res = &u * &u;

        assert_eq!(res, Ok(5.));
    }
}
