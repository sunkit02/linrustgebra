use error::{Error, Result};

pub mod error;
pub mod trait_impls;

#[derive(PartialEq, PartialOrd, Clone)]
pub struct Vector(Vec<f32>);

impl Vector {
    /// Zero vector by default
    pub fn new(len: usize) -> Self {
        Self(vec![0.; len])
    }

    pub fn from_iter<I: IntoIterator<Item = f32>>(iter: I) -> Self {
        let inner = iter.into_iter().collect();
        Self(inner)
    }

    pub fn to_vec(self) -> Vec<f32> {
        self.0
    }
}

impl Vector {
    /// Dimension of vector (number of entries)
    #[inline]
    pub fn dim(&self) -> usize {
        self.0.len()
    }

    /// Equal dimensions
    #[inline]
    pub fn dim_eq(&self, other: &Self) -> bool {
        self.0.len() == other.0.len()
    }

    /// Length of vector in terms of linear algebra not number of entries
    #[inline]
    pub fn length(&self) -> f32 {
        let sum = self.0.iter().map(|x| x * x).sum::<f32>() as f32;
        sum.sqrt()
    }

    #[inline]
    pub fn dot(&self, other: &Self) -> Result<f32> {
        self.validate_dimensions(other)?;

        let dot_product = self.0.iter().zip(other.0.iter()).map(|(x, y)| x * y).sum();
        Ok(dot_product)
    }

    #[inline]
    pub fn distance<V: AsRef<Self>>(&self, other: V) -> Result<f32> {
        let diff = (self - other.as_ref())?;
        Ok(diff.length())
    }

    #[inline]
    pub fn is_orthogonal_with(&self, other: &Self) -> Result<bool> {
        let res = self.dot(other)? == 0.;
        Ok(res)
    }
}

impl Vector {
    #[inline]
    fn validate_dimensions(&self, other: &Self) -> Result<()> {
        let self_dim = self.dim();
        let other_dim = other.dim();

        if self_dim != other_dim {
            return Err(Error::DimError {
                expected: self_dim,
                got: other_dim,
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_vectors() -> (Vector, Vector, Vector, Vector) {
        let u = Vector::from_iter([-1., 2.]);
        let v = Vector::from_iter([2., 3.]);
        let w = Vector::from_iter([3., -1., -5.]);
        let x = Vector::from_iter([6., -2., 3.]);

        (u, v, w, x)
    }

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
    fn can_calculate_square() {
        let (u, _, _, _) = init_vectors();
        let res = &u * &u;

        assert_eq!(res, Ok(5.));
    }

    #[test]
    fn can_calculate_dot_product() {
        let (u, v, _, _) = init_vectors();
        let res = u * &v;

        assert_eq!(res, Ok(4.));
    }

    #[test]
    fn can_calculate_length() {
        let (_, _, w, _) = init_vectors();
        let res = w.length();

        assert_eq!(res, f32::sqrt(35.0));
    }

    #[test]
    fn can_calculate_distance() {
        let (u, v, _, _) = init_vectors();
        let res = u.distance(v);

        assert_eq!(res, Ok(f32::sqrt(10.0)));
    }

    #[test]
    fn can_calculate_orthogonality() -> Result<()> {
        let u = Vector::from_iter([2., 4., 1.]);
        let v = Vector::from_iter([2., 1., -8.]);

        assert!(u.is_orthogonal_with(&v)?);

        Ok(())
    }
}
