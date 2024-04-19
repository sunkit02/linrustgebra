use linrustgebra::*;

use anyhow::Result;

fn main() -> Result<()> {
    let u = Vector::from_iter([-1., 2.]);
    let v = Vector::from_iter([2., 3.]);

    dbg!((u * &v)?);

    Ok(())
}
