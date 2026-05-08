use std::iter::Sum;

use num_traits::{FromPrimitive, One, Zero};

use crate::compute::{ComputeFloat, N};
use crate::error::{check_empty_set, is_valid_slice};
use crate::numeric::{NumExt, NumOps};
use crate::{DataSet, Marker, Population, Result};
use crate::{Numeric, StatsError};

use super::std_dev;

pub trait Correlation<T, M>
where
    M: Marker,
{
    type Output;
    fn covariance(&self, other: &[T]) -> Result<Self::Output>;
    fn pearson(&self, other: &[T]) -> Result<Self::Output>;
    //   fn spearman_r(&self, other: &DataSet<T, M>) -> Result<f64>;
    //   fn autocorrelation(&self, lag: usize) -> Result<f64>;
    //   fn r_squared(&self, other: &DataSet<T, M>) -> Result<f64>;
}

fn pearson<T, M>(xs: &[T], ys: &[T]) -> Result<T>
where
    M: Marker,
    T: NumOps + NumExt + Zero + One + FromPrimitive,
{
    let dof = is_valid_slice::<T, M>(xs)?;
    let cov = covariance::<T, M>(xs, ys)?;
    let sx = std_dev::<T, M>(xs)?;
    let sy = std_dev::<T, M>(ys)?;
    let res = cov / (sx * sy);
    Ok(res)
}

pub fn covariance<T, M>(xs: &[T], ys: &[T]) -> Result<T>
where
    M: Marker,
    T: NumOps + Zero + One + FromPrimitive,
{
    if xs.len() != ys.len() {
        return Err(StatsError::LengthMismatch {
            len_a: xs.len(),
            len_b: ys.len(),
        });
    }

    let nuterlized_len = is_valid_slice::<T, M>(xs)?;

    let mut n = T::zero();
    let mut mean_x = T::zero();
    let mut mean_y = T::zero();
    let mut c2 = T::zero();

    for (&x, &y) in xs.iter().zip(ys) {
        n += T::one();
        let dx_old = x - mean_x;
        mean_x += dx_old / n;
        let dy_old = y - mean_y;
        mean_y += dy_old / n;
        c2 += dx_old * (y - mean_y); // old dx, new mean_y
    }

    Ok(c2 / nuterlized_len)
}

impl<T> Correlation<T, Population> for [T]
where
    T: NumOps + NumExt + Zero + One + FromPrimitive,
{
    type Output = T;

    fn covariance(&self, other: &[T]) -> Result<Self::Output> {
        covariance::<T, Population>(self, other)
    }

    fn pearson(&self, other: &[T]) -> Result<Self::Output> {
        pearson::<T, Population>(self, other)
    }
}

#[cfg(test)]
mod test {
    use crate::stats::correlation::{Correlation, pearson};
    use crate::stats::covariance;
    use crate::{DataSet, Sample};

    #[test]
    fn covariance_t() {
        let series = [1, 2, 3, 4, 5, 6, 7, 8, 9].map(|x| x as f64);
        let other = [10, 20, 27, 13, 32, 12, 89, 66, 43].map(|x| x as f64);
        let cov = covariance::<f64, Sample>(&series, &other).unwrap();
        assert_eq!(cov, 49.125);
    }

    #[test]
    fn pearson_t() {
        let series = [1, 2, 3, 4, 5, 6, 7, 8, 9].map(|x| x as f64);
        let other = [10, 20, 27, 13, 32, 12, 89, 66, 43].map(|x| x as f64);
        let p = pearson::<f64, Sample>(&series, &other).unwrap();
        assert_eq!(p, 0.6618750825608319);
    }
}
