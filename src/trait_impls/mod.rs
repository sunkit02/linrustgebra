use std::fmt::Debug;

use crate::Vector;

mod operators;

impl std::fmt::Debug for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<Vector> for Vector {
    fn as_ref(&self) -> &Vector {
        self
    }
}
