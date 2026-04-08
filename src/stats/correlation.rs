use crate::compute::{ComputeFloat, N};
use crate::error::check_empty_set;
use crate::{DataSet, Marker, Result};
use crate::{Numeric, StatsError};

use super::std_dev_n;

pub trait Correlation<T: Numeric, M: Marker> {
    fn covariance(&self, other: &DataSet<T, M>) -> Result<f64>;
    fn pearson(&self, other: &DataSet<T, M>) -> Result<f64>;
    //   fn spearman_r(&self, other: &DataSet<T, M>) -> Result<f64>;
    //   fn autocorrelation(&self, lag: usize) -> Result<f64>;
    //   fn r_squared(&self, other: &DataSet<T, M>) -> Result<f64>;
}

impl<T: Numeric, M: Marker> Correlation<T, M> for DataSet<T, M> {
    fn covariance(&self, other: &DataSet<T, M>) -> Result<f64> {
        let dof = self.dof_denominator_n()?;
        let xs = self.to_n_vec()?;
        let ys = other.to_n_vec()?;
        Ok(covariance_n(&xs, &ys, dof)?.cf_to_f64())
    }

    fn pearson(&self, other: &DataSet<T, M>) -> Result<f64> {
        let dof = self.dof_denominator_n()?;
        let xs = self.to_n_vec()?;
        let ys = other.to_n_vec()?;

        let cov = covariance_n(&xs, &ys, dof)?;
        let (_, _, sx) = std_dev_n(&xs, dof)?;
        let (_, _, sy) = std_dev_n(&ys, dof)?;
        let res = cov / (sx * sy);
        Ok(res.cf_to_f64())
    }
}

// pub(crate) fn covariance_n(xs: &[N], ys: &[N], dof: N) -> N {
//     let mu_x = mean_n(xs);
//     let mu_y = mean_n(ys);
//     let sum = N::cf_sum(xs.iter().zip(ys).map(|(&x, &y)| (x - mu_x) * (y - mu_y)));
//     sum / dof
// }
pub(crate) fn covariance_n(xs: &[N], ys: &[N], dof: N) -> Result<N> {
    if xs.len() != ys.len() {
        return Err(StatsError::LengthMismatch {
            len_a: xs.len(),
            len_b: ys.len(),
        });
    }

    check_empty_set(xs)?;

    let mut n = N::cf_zero();
    let mut mean_x = N::cf_zero();
    let mut mean_y = N::cf_zero();
    let mut c2 = N::cf_zero();

    for (&x, &y) in xs.iter().zip(ys) {
        n += N::cf_one();
        let dx_old = x - mean_x;
        mean_x += dx_old / n;
        let dy_old = y - mean_y;
        mean_y += dy_old / n;
        c2 += dx_old * (y - mean_y); // old dx, new mean_y
    }

    Ok(c2 / dof)
}

#[cfg(test)]
mod test {
    use crate::{Correlation, DataSet, Sample};

    #[test]
    fn covariance_t() {
        let series: DataSet<i32, Sample> = DataSet::from_iter([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let other: DataSet<i32, Sample> = DataSet::from_iter([10, 20, 27, 13, 32, 12, 89, 66, 43]);
        let cov = series.covariance(&other).unwrap();
        assert_eq!(cov, 49.125);
    }

    #[test]
    fn pearson_t() {
        let series: DataSet<i32, Sample> = DataSet::from_iter([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let other: DataSet<i32, Sample> = DataSet::from_iter([10, 20, 27, 13, 32, 12, 89, 66, 43]);
        let p = series.pearson(&other).unwrap();
        assert_eq!(p, 0.6618750825608319);
    }
}
